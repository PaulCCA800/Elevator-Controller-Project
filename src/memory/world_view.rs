use crossbeam_channel as cbc;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant};

use crate::memory::elevator::{DeadOrAlive, Behaviour, Obstruction, ElevatorDirection, Elevator};
use crate::memory::hall_order_queue::HallOrderQueue;
use crate::memory::order::{Order, OrderStatus, order_status_rank};


#[derive(Debug)]
pub enum MemoryCommand {
    ElevatorStatus(ElevatorStatusCommand),
    HallOrderQueue(HallOrderQueueCommand),
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ElevatorStatusCommand {
    SetDeadOrAlive {elevator_id: u16, dead_or_alive: DeadOrAlive},
    SetBehaviour {elevator_id: u16, behavior: Behaviour},
    SetObstruction {elevator_id: u16, obstruction: Obstruction},
    SetFloor {elevator_id: u16, floor: u8},
    SetDirection {elevator_id: u16, dir: ElevatorDirection},
    SetCabOrders {elevator_id: u16, orders: VecDeque<Order>},
    AddCabOrder {elevator_id: u16, order: Order},
    RemoveCabOrder {elevator_id: u16},
    SetCabOrderStatus {elevator_id: u16, order_id: u16, status: OrderStatus}, 
    SynchronizeWorldView {world_view: WorldView},
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum HallOrderQueueCommand {
    AddToHallOrderQueue {order: Order},
    RemoveFromHallOrderQueue{order_id: u16},
    SetHallOrderStatus{order_id: u16, status: OrderStatus},
    SetAckBarrier{order_id: u16, barrier: Vec<u16>},
    InsertIntoAckBarrier{order_id: u16, elevator_id: u16},
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct WorldView {
    my_elevator_id: u16,
    my_session_id: u16,
    elevator_statuses: HashMap <u16, Elevator>,
    hall_order_queue: HallOrderQueue,
    write_counters: HashMap <u16, u16>,
    recorded_session_ids: HashMap<u16, u16>,
}


impl WorldView {

    pub fn new(my_elevator_id: u16, my_session_id: u16) -> Self {
        
        Self{
            my_elevator_id,
            my_session_id,
            elevator_statuses: Self::initialize_elevator_statuses(my_elevator_id, my_session_id),
            hall_order_queue: HallOrderQueue::new(),
            write_counters: Self::initialize_write_counter(my_elevator_id),
            recorded_session_ids: Self::initialize_recorded_session_ids(my_elevator_id, my_session_id),
            }
    }


    fn initialize_elevator_statuses(elevator_id: u16, session_id: u16) -> HashMap<u16, Elevator> {
        let mut initial_elevator_statuses = HashMap::new();
        initial_elevator_statuses.insert(elevator_id, Elevator::new(elevator_id, session_id));

        return initial_elevator_statuses
    }


    fn initialize_write_counter(id: u16) -> HashMap<u16, u16> {
        let mut initial_write_counter = HashMap::new();
        initial_write_counter.insert(id, 0);

        return initial_write_counter
    }


    fn initialize_recorded_session_ids(elevator_id: u16, session_id: u16) -> HashMap<u16, u16> {
        let mut recorded_session_ids: HashMap<u16, u16> = HashMap::new();
        recorded_session_ids.insert(elevator_id, session_id);
        
        return recorded_session_ids;
    }


    pub fn get_id(&self) -> u16 {
        return self.my_elevator_id
    }


    fn get_session_id(&self) -> u16 {
        return self.my_session_id
    }


    pub fn get_elevator_statuses(&self) -> &HashMap<u16, Elevator> {
        return &self.elevator_statuses;
    }


    pub fn get_hall_order_queue(&self) -> &HallOrderQueue {
        return &self.hall_order_queue
    }


    pub fn get_mut_hall_order_queue(&mut self) -> &mut HallOrderQueue {
        return &mut self.hall_order_queue
    }


    fn add_to_hall_order_queue(&mut self, mut order: Order) {
        if !order.get_ack_barrier().contains(&self.get_id()) {
            order.insert_into_ack_barrier(self.get_id());
        }
        self.get_mut_hall_order_queue().add_to_queue(order)
    }


    fn set_hall_order_status(&mut self, order_id: u16, status: OrderStatus) { 
        let my_id = self.get_id();
        if let Some(order) = self.hall_order_queue.get_mut_hall_order(order_id) {
            order.set_order_status(status);
            if !order.get_ack_barrier().contains(&my_id) {
                order.insert_into_ack_barrier(my_id);
            }    
        }
    }


    fn get_recorded_session_ids(&self) -> &HashMap<u16, u16> {
        return &self.recorded_session_ids
    }


    fn get_mut_recorded_session_ids(&mut self) -> &mut HashMap<u16, u16> {
        return &mut self.recorded_session_ids
    }


    pub fn get_elevator(&self, elevator_id: u16) -> &Elevator {
        return self.elevator_statuses.get(&elevator_id)
        .expect(&format!("get error: no elevator found at {}.", elevator_id));
    }


    pub fn get_mut_elevator(&mut self, elevator_id: u16) -> &mut Elevator {
        return self.elevator_statuses.get_mut(&elevator_id)
        .expect(&format!("get_mut error: no elevator found at {}.", elevator_id));
    }


    fn set_elev_dead_or_alive(&mut self, elevator_id: u16, dead_or_alive: DeadOrAlive) {
        self.get_mut_elevator(elevator_id).set_dead_or_alive(dead_or_alive);
    }


    fn set_elev_current_floor(&mut self, elevator_id: u16, floor: u8) {
        self.get_mut_elevator(elevator_id).set_floor(floor);
    }


    fn set_elev_direction(&mut self, elevator_id: u16, direction: ElevatorDirection) {
        self.get_mut_elevator(elevator_id).set_direction(direction);
    }


    pub fn get_elev_behaviour(&self, elevator_id: u16) -> &Behaviour {
        return self.get_elevator(elevator_id).get_behaviour();
    }


    fn set_elev_behaviour(&mut self, elevator_id: u16, behaviour: Behaviour) {
        self.get_mut_elevator(elevator_id).set_behavior(behaviour);
    }


    pub fn get_elev_obstruction(&self, elevator_id: u16) -> &Obstruction {
        return self.get_elevator(elevator_id).get_obstruction();
    }


    fn set_elev_obstruction(&mut self, elevator_id: u16, obstruction: Obstruction) {
        self.get_mut_elevator(elevator_id).set_obstruction(obstruction);
    }


    pub fn set_elev_cab_orders(&mut self, elevator_id: u16, orders: VecDeque<Order>) {
        self.get_mut_elevator(elevator_id).set_cab_orders(orders);
    }


    fn add_elev_cab_order(&mut self, elevator_id: u16, mut order: Order) {
        if !order.get_ack_barrier().contains(&self.get_id()) {
            order.insert_into_ack_barrier(self.get_id());
        }
        self.get_mut_elevator(elevator_id).add_cab_order(order);
    }


    fn remove_elev_cab_order(&mut self, elevator_id: u16) {
        self.get_mut_elevator(elevator_id).remove_cab_order();
    }


    fn set_elev_cab_order_status(&mut self, 
        elevator_id: u16, 
        order_id: u16, 
        status: OrderStatus
    ) {
        let my_id = self.get_id();

        if let Some(order) = self
        .get_mut_elevator(elevator_id)
        .get_mut_cab_orders()
        .iter_mut()
        .find(|o| *o.get_order_id() == order_id)
        {
            order.set_order_status(status);

            if !order.get_ack_barrier().contains(&my_id) {
                order.insert_into_ack_barrier(my_id);
            }
        }
    }


    fn get_write_counters(&self) -> &HashMap<u16, u16> {
        return &self.write_counters
    }


    fn get_mut_write_counters(&mut self) -> &mut HashMap<u16, u16> {
        return &mut self.write_counters
    }


    fn get_elev_write_counter(&self, elevator_id: u16) -> u16 {
        match self.get_write_counters().get(&elevator_id) {
            Some(write_counter) => return *write_counter,
            None => return 0
        }
    }


    fn set_write_counter(&mut self, elevator_id: u16, value: u16) {
        self.get_mut_write_counters().insert(elevator_id, value);
    }
 

    fn increment_write_counter(&mut self, elevator_id: &u16) {
        let counter = self.get_mut_write_counters().entry(*elevator_id).or_insert(0);
        *counter = counter.wrapping_add(1);
    }


    fn add_new_write_counter(&mut self, incoming_world_view: &WorldView, elevetor_id: u16) {
        if !self.get_write_counters().keys().any(|k| k == &elevetor_id) {
                let incoming_write_counter: u16 = match incoming_world_view.get_write_counters().get(&elevetor_id) {
                    Some(counter) => *counter,
                    None => 0,
                };
            self.get_mut_write_counters().insert(elevetor_id, incoming_write_counter);
        }
    }


    fn cab_order_status_manager(&mut self, num_elevators: u8) {
        let mut orders_to_remove: Vec<Order> = Vec::new();

        let my_id = self.get_id();
        let my_cab_orders = self.get_mut_elevator(my_id).get_mut_cab_orders();

        for order in my_cab_orders.iter_mut() {

            let unique_elevator_ids_count = order
            .get_ack_barrier()
            .iter()
            .collect::<HashSet<_>>()
            .len();

            if order.get_order_status() == &OrderStatus::Unconfirmed 
            && unique_elevator_ids_count == num_elevators as usize {
                order.set_order_status(OrderStatus::Confirmed);
                order.get_mut_ack_barrier().clear();
            }

            else if order.get_order_status() == &OrderStatus::Completed 
            && unique_elevator_ids_count == num_elevators as usize {
                order.set_order_status(OrderStatus::ReadyForDeletion);
                order.get_mut_ack_barrier().clear();
            }

            else if order.get_order_status() == &OrderStatus::ReadyForDeletion 
            && unique_elevator_ids_count == num_elevators as usize {
                orders_to_remove.push(order.clone());
            }
        }
        my_cab_orders.retain(|o| !orders_to_remove.contains(o));    
    }


    fn update_my_hall_order_queue(
        &mut self,
        incoming_elevator_id: u16,
        incoming_world_view: &WorldView,
        me_resurrected: bool,
        incoming_resurrected: bool,
    ) {
        if self.is_counter_newer(
            &incoming_elevator_id, 
            incoming_world_view, 
            me_resurrected
        ) && !incoming_resurrected {

            let my_id: u16 = self.get_id();

            let my_pending_cleanups = 
            self
            .get_hall_order_queue()
            .get_pending_cleanup()
            .clone();

            let my_hall_order_queue: &mut HashMap<u16, Order> =
                self.get_mut_hall_order_queue().get_mut_hall_order_queue();

            let incoming_hall_order_queue: &HashMap<u16, Order> =
            incoming_world_view.get_hall_order_queue().get_order_queue();

            let incoming_hall_order_ids: Vec<u16> = 
            incoming_hall_order_queue
            .keys()
            .cloned()
            .collect();

            for id in incoming_hall_order_ids {
                
                if my_pending_cleanups.contains(&id) {
                    continue;
                }

                let mut incoming_order: Order = incoming_world_view
                    .get_hall_order_queue()
                    .get_hall_order(id)
                    .clone();

                match my_hall_order_queue.get(&id) {
                    None => {
                        incoming_order.insert_into_ack_barrier(my_id);
                        my_hall_order_queue.insert(id, incoming_order);
                    }

                    Some(my_order) => {
                        let my_status: &OrderStatus = my_order.get_order_status();
                        let incoming_status: &OrderStatus = incoming_hall_order_queue
                            .get(&id)
                            .unwrap_or_else(|| panic!("critical error retrieving hall order {}", id))
                            .get_order_status();

                        if order_status_rank(incoming_status) > order_status_rank(my_status) {
                            incoming_order.insert_into_ack_barrier(my_id);
                            my_hall_order_queue.insert(id, incoming_order);
                        }
                    }
                }
            }
        }
    }


    fn update_my_write_counters(
        &mut self, 
        world_view: &WorldView, 
        me_resurrected: bool, 
        other_resurrected: bool
    ) {

        let incoming_write_counters = world_view.get_write_counters();
        for elevator_id in incoming_write_counters.keys(){
            if self.is_counter_newer(elevator_id, world_view, me_resurrected) && !other_resurrected {
                self.set_write_counter(*elevator_id, world_view.get_elev_write_counter(*elevator_id));
            }
        }
    }


    fn update_cab_ack_barriers(
        &mut self, 
        incoming_elevator_id: u16, 
        incoming_world_view: &WorldView, 
        me_resurrected: bool, 
        incoming_resurrected: bool
    ) {
        let my_id = self.get_id();

        if self.is_counter_newer(
            &incoming_elevator_id, 
            incoming_world_view, 
            me_resurrected) 
            && !incoming_resurrected {

            if let Some(incoming_elevator) = incoming_world_view
            .get_elevator_statuses()
            .get(&incoming_elevator_id) {

                let incoming_cab_orders = incoming_elevator.get_cab_orders();
                for order in incoming_cab_orders {

                    if !order.get_ack_barrier().contains(&my_id) {

                        let my_local_cab_orders = self
                        .get_mut_elevator(incoming_elevator_id)
                        .get_mut_cab_orders();

                        if let Some(my_matching_order) = my_local_cab_orders
                        .iter_mut()
                        .find(|o| o.get_order_id() == order.get_order_id()) {
                            my_matching_order.insert_into_ack_barrier(my_id);
                        }   
                    }
                }
            }

            if let Some(incoming_wv_my_elevator) = incoming_world_view.get_elevator_statuses().get(&my_id) {

                let incoming_wv_my_cab_orders = incoming_wv_my_elevator.get_cab_orders();
                let my_local_elevator = self.get_mut_elevator(my_id);
                let my_local_cab_orders = my_local_elevator.get_mut_cab_orders();

                for order in incoming_wv_my_cab_orders {

                    if let Some(my_matching_order) = 
                    my_local_cab_orders
                    .iter_mut()
                    .find(|o| o.get_order_id() == order.get_order_id()){

                        for id in order.get_ack_barrier(){
                            if !my_matching_order.get_ack_barrier().contains(id){
                                my_matching_order.insert_into_ack_barrier(*id);
                            }
                        }
                    }
                }
            }
        }
    }


    fn update_hall_ack_barriers(
        &mut self,
        incoming_elevator_id: u16,
        incoming_world_view: &WorldView,
        me_resurrected: bool,
        other_resurrected: bool,
    ) {
        if self.is_counter_newer(
            &incoming_elevator_id, 
            incoming_world_view, 
            me_resurrected) && !other_resurrected {

            let incoming_hall_order_queue: &HashMap<u16, Order> =
                incoming_world_view.hall_order_queue.get_order_queue();

            for order in incoming_hall_order_queue.values() {
                let order_id: u16 = *order.get_order_id();
                let elevator_ids_in_other_barrier: Vec<u16> =
                    order.get_ack_barrier().iter().cloned().collect();

                if let Some(my_order) = self.hall_order_queue.get_mut_hall_order(order_id) {
                    for id in elevator_ids_in_other_barrier {
                        if !my_order.get_ack_barrier().contains(&id) {
                            my_order.insert_into_ack_barrier(id);
                        }
                    }
                }
            }
        }
    }


    fn is_counter_newer(&mut self, 
        incoming_elevator_id: &u16, 
        incoming_world_view: &WorldView, 
        me_resurrected: bool
    ) -> bool {

        let incoming_write_counter: &u16 = match incoming_world_view.get_write_counters().get(incoming_elevator_id){
                Some(counter) => counter,
                None => return false,
            };

        let my_last_recorded_counter: &mut u16 = self
        .get_mut_write_counters()
        .entry(*incoming_elevator_id)
        .or_insert(0); 

        let version_control = incoming_write_counter.wrapping_sub(*my_last_recorded_counter);
        
        let mut is_newer: bool = false;
        match me_resurrected{
            false => {
                if version_control != 0 
                && version_control < (1 << 15) {
                    is_newer = true
                }
            }
            true => {
                if version_control < (1 << 15) {
                    is_newer = true
                }
            }  
        }
        return is_newer
    }


    fn synchronize_world_view(&mut self, incoming_world_view: WorldView) {

        let my_id: u16 = self.get_id();
        let my_session_id:u16 = self.get_session_id();

        let other_id: u16 = incoming_world_view.get_id();
        let other_session_id: u16 = incoming_world_view.get_session_id();

        let last_recorded_session_ids: &mut HashMap<u16, u16> = self.get_mut_recorded_session_ids();
        let incoming_last_recorded_session_ids: &HashMap<u16, u16> = incoming_world_view.get_recorded_session_ids();

        let mut incoming_resurrected: bool = false;
        let mut me_resurrected: bool = false;

        if !last_recorded_session_ids.keys().any(|k| *k == other_id) {
            last_recorded_session_ids.insert(other_id, other_session_id);
        }

        if last_recorded_session_ids.get(&other_id).expect("critical error synchronizing session ids.") != &other_session_id {
            last_recorded_session_ids.insert(other_id, other_session_id);
            incoming_resurrected = true;
        }

        if let Some(session_id) = incoming_last_recorded_session_ids.get(&my_id) {
            if session_id != &my_session_id{
                me_resurrected = true
            };
        }

        for id in incoming_last_recorded_session_ids.keys() {

            if *id == my_id && !me_resurrected {
                continue;
            }

            self.add_new_write_counter(&incoming_world_view, *id);

            let is_newer = self.is_counter_newer(id, &incoming_world_view, me_resurrected);

            if is_newer && !incoming_resurrected {
                if !self.elevator_statuses.contains_key(id) {
                    self.elevator_statuses.insert(*id, incoming_world_view.get_elevator(*id).clone());
                }

                let my_stored_elevator: &mut Elevator = self.get_mut_elevator(*id);
                let incoming_stored_elevator: &Elevator = incoming_world_view.get_elevator(*id);

                my_stored_elevator.set_behavior(*incoming_stored_elevator.get_behaviour());
                my_stored_elevator.set_obstruction(*incoming_stored_elevator.get_obstruction());
                my_stored_elevator.set_floor(*incoming_stored_elevator.get_floor());
                my_stored_elevator.set_direction(*incoming_stored_elevator.get_direction());
                my_stored_elevator.set_cab_orders(incoming_stored_elevator.get_cab_orders().clone());
                my_stored_elevator.set_dead_or_alive(*incoming_stored_elevator.get_dead_or_alive());
                self.update_cab_ack_barriers(*id, &incoming_world_view, me_resurrected, incoming_resurrected);
            }
        }

        self.update_my_hall_order_queue(
            other_id, 
            &incoming_world_view, 
            me_resurrected, 
            incoming_resurrected, 
        );
        self.update_hall_ack_barriers(
            other_id, 
            &incoming_world_view, 
            me_resurrected, 
            incoming_resurrected
        );
        self.update_my_write_counters(
            &incoming_world_view, 
            me_resurrected, 
            incoming_resurrected
        );
        self.increment_write_counter(&my_id);
    }


    fn alive_elevators_count(&self) -> u8 {
        let mut elevators_alive_count: u8 = 0;
        for elevator in self.get_elevator_statuses().values(){
            if elevator.get_dead_or_alive() == &DeadOrAlive::Alive{
                elevators_alive_count += 1;
            }
        }
        elevators_alive_count.max(1)
    }


    fn maintain_order_statuses(&mut self) {

        let elevators_alive_count = self.alive_elevators_count();
        let my_id: u16 = self.get_id();

        self.get_mut_hall_order_queue()
        .hall_order_status_manager(
elevators_alive_count, 
            my_id
        );
        self.cab_order_status_manager(elevators_alive_count);
    }


    pub fn edit_elevator_status(&mut self, command: ElevatorStatusCommand) {

        match command {
            ElevatorStatusCommand::SetDeadOrAlive {elevator_id, dead_or_alive}
            => self.set_elev_dead_or_alive(elevator_id, dead_or_alive),

            ElevatorStatusCommand::SetFloor {elevator_id, floor}
            => self.set_elev_current_floor(elevator_id, floor),

            ElevatorStatusCommand::SetDirection {elevator_id, dir}
            => self.set_elev_direction(elevator_id, dir),

            ElevatorStatusCommand::SetObstruction {elevator_id, obstruction}
            => self.set_elev_obstruction(elevator_id, obstruction),

            ElevatorStatusCommand::SetBehaviour {elevator_id, behavior}
            => self.set_elev_behaviour(elevator_id, behavior),

            ElevatorStatusCommand::SetCabOrders {elevator_id, orders}
            => self.set_elev_cab_orders(elevator_id, orders),

            ElevatorStatusCommand::AddCabOrder {elevator_id, order}
            => self.add_elev_cab_order(elevator_id, order),

            ElevatorStatusCommand::RemoveCabOrder {elevator_id}
            => self.remove_elev_cab_order(elevator_id),

            ElevatorStatusCommand::SetCabOrderStatus {elevator_id, order_id, status}
            => self.set_elev_cab_order_status(elevator_id, order_id, status),

            ElevatorStatusCommand::SynchronizeWorldView {world_view}
            => self.synchronize_world_view(world_view),

        }
    }


    pub fn edit_hall_order_queue(&mut self, command: HallOrderQueueCommand) {

        match command {
            HallOrderQueueCommand::AddToHallOrderQueue {mut order}
            => self.add_to_hall_order_queue(order),

            HallOrderQueueCommand::RemoveFromHallOrderQueue {order_id}
            => self.hall_order_queue.remove_from_queue(order_id),

            HallOrderQueueCommand::SetHallOrderStatus {order_id, status}
            => self.set_hall_order_status(order_id, status),

            HallOrderQueueCommand::SetAckBarrier {order_id, barrier}
            => match self.hall_order_queue.get_mut_hall_order(order_id){
                   Some(order) => order.set_ack_barrier(barrier),
                   None => {},
            },

            HallOrderQueueCommand::InsertIntoAckBarrier {order_id, elevator_id}
            => match self.hall_order_queue.get_mut_hall_order(order_id){
                   Some(order) => order.insert_into_ack_barrier(elevator_id),
                   None => {},
            },
        }
    }
}


pub fn memory_thread(
    my_elevator_id: u16,
    my_session_id: u16,
    rx_memory: cbc::Receiver<MemoryCommand>,
    rx_network: cbc::Receiver<WorldView>,
    tx_elevator_state: cbc::Sender<Elevator>,
    tx_decision: cbc::Sender<WorldView>,
    tx_network_tx: cbc::Sender<WorldView>,) {

    let mut my_local_world_view = WorldView::new(my_elevator_id, my_session_id);

    let mut last_seen_timers: HashMap<u16, Instant> = HashMap::new();
    let timeout = Duration::from_secs(4);
    last_seen_timers.insert(my_elevator_id, Instant::now());
    let dead_or_alive_ticker = cbc::tick(Duration::from_millis(50));

    let cleanup_ticker = cbc::tick(Duration::from_secs(5));
    
    let mut last_sent_elevator: Option<Elevator> = None;
    let mut last_sent_world_view: Option<WorldView> = None;

    loop {
        cbc::select! {
            recv(rx_network) -> message => {
                match message {
                    Ok(world_view) => {
                        let sender_id = world_view.get_id();

                        my_local_world_view.edit_elevator_status(
                            ElevatorStatusCommand::SynchronizeWorldView {world_view}
                        );

                        last_seen_timers.insert(sender_id, Instant::now());

                        if my_local_world_view.get_elevator_statuses().contains_key(&sender_id) {
                            my_local_world_view
                                .get_mut_elevator(sender_id)
                                .set_dead_or_alive(DeadOrAlive::Alive);
                        }
                    }
                    Err(_) => {
                        print!("failed to receive on rx_network for elevator {}", my_elevator_id);
                    }
                }
            }

            recv(rx_memory) -> message => {
                match message {
                    Ok(command) => {
                        match command {
                            MemoryCommand::ElevatorStatus(cmd) => {
                                my_local_world_view.edit_elevator_status(cmd);
                            }
                            MemoryCommand::HallOrderQueue(cmd) => {
                                my_local_world_view.edit_hall_order_queue(cmd);
                            }
                        }
                    }
                    Err(_) => {
                        print!("failed to receive on rx_memory for elevator {}", my_elevator_id);
                    }
                }
            }

            recv(dead_or_alive_ticker) -> _ => {
                let mut dead_ids = Vec::new();

                for (id, last_seen) in &last_seen_timers {
                    if *id != my_elevator_id && last_seen.elapsed() > timeout {
                        dead_ids.push(*id);
                    }
                }

                for id in dead_ids {
                    if my_local_world_view.get_elevator_statuses().contains_key(&id) {
                        my_local_world_view
                            .get_mut_elevator(id)
                            .set_dead_or_alive(DeadOrAlive::Dead);
                    }
                }
            }
            
            recv(cleanup_ticker) -> _ => {
                let order_ids: Vec<u16> = my_local_world_view
                    .get_mut_hall_order_queue()
                    .get_mut_pending_cleanup()
                    .drain()
                    .collect();

                for order_id in order_ids {
                    my_local_world_view.get_mut_hall_order_queue().remove_from_queue(order_id);
                }
            }
        }

        my_local_world_view.maintain_order_statuses();

        let elevator_snapshot = my_local_world_view.get_elevator(my_elevator_id).clone();
        if last_sent_elevator.as_ref() != Some(&elevator_snapshot) {
            tx_elevator_state.send(elevator_snapshot.clone()).unwrap();
            last_sent_elevator = Some(elevator_snapshot);
        }

        let world_view_snapshot = my_local_world_view.clone();
        if last_sent_world_view.as_ref() != Some(&world_view_snapshot) {
            tx_decision.send(world_view_snapshot.clone()).unwrap();
            tx_network_tx.send(world_view_snapshot.clone()).unwrap();
            last_sent_world_view = Some(world_view_snapshot);
        }
    }
}
