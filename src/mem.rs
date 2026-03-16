use std::{collections::{HashMap, HashSet, VecDeque}, hash::Hash};
use serde::{Deserialize, Serialize};
use rand::Rng;

use crate::misc::generate_id;

#[derive(Copy,Clone,Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum DeadOrAlive {
    Dead,
    Alive,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Behaviour {
    Idle,
    Moving,
    DoorOpen,
}

#[derive(Copy, Clone)]
pub enum OrderType {
    Cab,
    Hall,
}

#[derive(Copy, Clone)]
pub enum Obstruction {
    Obstructed,
    Clear,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElevatorDirection {
    Up,
    Down,
    Stop,
}

#[derive(Copy, Clone, Debug)]
pub enum OrderDirection {
    Up,
    Down,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OrderStatus {
    Unconfirmed,
    Confirmed,
    Completed, 
    ReadyForDeletion,
}

#[derive(Clone)]
pub struct Order {
    order_id: u64,
    floor: u8,
    order_type: OrderType,
    direction: OrderDirection,
    order_status: OrderStatus,
    ack_barrier: Vec<u64>,
}

#[derive(Clone)]
pub struct Elevator {
    dead_or_alive: DeadOrAlive,
    elevator_id: u64,
    session_id: u64,
    behaviour: Behaviour,
    obstruction: Obstruction,
    floor: u8,
    direction: ElevatorDirection,
    cab_requests: VecDeque<Order>,
}

#[derive(Clone)]
pub struct WorldView {
    my_elevator_id: u64,
    session_id: u64,
    elevator_statuses: HashMap <u64, Elevator>,
    hall_order_queue: HashMap<u64, Order>,
    write_counter: HashMap <u64, u64>,
    heart_beat_counters: HashMap<u64, u16>,
    recorded_session_ids: HashMap<u64, u64>,
}

pub type HallOrders = VecDeque<Order>;

pub enum ElevatorStatusCommand {
    SetBehaviour {elevator_id: u64, behavior: Behaviour},
    SetObstruction {elevator_id: u64, obstruction: Obstruction},
    SetFloor {elevator_id: u64, floor: u8},
    SetDirection {elevator_id: u64, dir: ElevatorDirection},
    SetCabRequests {elevator_id: u64, orders: VecDeque<Order>},
    AddCabRequest {elevator_id: u64, order: Order},
    RemoveCabRequest {elevator_id: u64},
    SynchronizeWorldView {world_view: WorldView},
}

pub enum OrderQueueCommand {
    AddToOrderQueue {order: Order},
    RemoveFromOrderQueue{order_id: u64},
    SetOrderStatus{order_id: u64, status: OrderStatus},
    SetAckBarrier{order_id: u64, barrier: Vec<u64>},
    InsertAckBarrier{order_id: u64, elevator_id: u64},
}

impl Order {
    pub fn new(floor: u8, order_type: OrderType, direction: OrderDirection) -> Self{
        Self{
            order_id: Self::generate_order_ID(),
            floor,
            order_type,
            direction,
            order_status: OrderStatus::Unconfirmed,
            ack_barrier: Vec::new(),
        }
    }

    fn generate_order_ID() -> u64 {
        return rand::random();
    }

    pub fn get_order_id(&self) -> &u64 {
        return &self.order_id
    }

    pub fn get_floor(&self) -> &u8 {
        return &self.floor
    }

    pub fn get_order_type(&self) -> &OrderType {
        return &self.order_type
    }

    pub fn get_direction(&self) -> &OrderDirection {
        return &self.direction
    }

    pub fn get_order_status(&self) -> &OrderStatus {
        return &self.order_status
    }
    
    pub fn set_order_status(&mut self, status: OrderStatus) {
        self.order_status = status;
    }

    pub fn get_ack_barrier(&self) -> &Vec<u64>{
        return &self.ack_barrier
    }

    pub fn get_mut_ack_barrier(&mut self) -> &mut Vec<u64>{
        return &mut self.ack_barrier
    }

    pub fn set_ack_barrier(&mut self, barrier: Vec<u64>) {
        self.ack_barrier = barrier;
    }

    pub fn insert_into_ack_barrier(&mut self, elevator_id: u64) {
        self.ack_barrier.push(elevator_id);
    }
}

impl Elevator{
    pub fn new(elevator_id: u64) -> Self{
        Self{
            dead_or_alive: DeadOrAlive::Alive,
            elevator_id,
            session_id: Self::generate_session_id(),
            behaviour: Behaviour::Idle,
            obstruction: Obstruction::Clear,
            floor: 1,
            direction: ElevatorDirection::Stop,
            cab_requests: Self::initialize_cab_requests(),
        }
    }

    fn initialize_cab_requests() -> VecDeque<Order>{
        return VecDeque::new();
    }

    fn generate_session_id() -> u64 {
        return rand::random();
    }

    pub fn get_dead_or_alive(&self) -> &DeadOrAlive{
        return &self.dead_or_alive
    }

    pub fn set_dead_or_alive(&mut self, status: DeadOrAlive){
        self.dead_or_alive = status
    }

    pub fn get_elevator_id(&self) -> &u64{
        return &self.elevator_id
    }

    pub fn get_session_id(&self) -> &u64{
        return &self.session_id
    }

    pub fn get_behaviour(&self) -> &Behaviour{
        return &self.behaviour
    }

    pub fn set_behavior(&mut self, behaviour: Behaviour) {
        self.behaviour = behaviour
    }

    pub fn get_obstruction(&self) -> &Obstruction{
        return &self.obstruction
    }

     pub fn set_obstruction(&mut self, obstruction: Obstruction) {
        self.obstruction = obstruction
    }

    pub fn get_floor(&self) -> &u8{
        return &self.floor
    }

    pub fn set_floor(&mut self, floor: u8) {
        self.floor = floor;
    }

    pub fn get_direction(&self) -> &ElevatorDirection {
        return &self.direction
    }

    pub fn set_direction(&mut self, dir: ElevatorDirection) {
        self.direction = dir;
    }

    pub fn get_cab_requests(&self) -> &VecDeque<Order>{
        &self.cab_requests
    }

    pub fn get_mut_cab_requests(&mut self) -> &mut VecDeque<Order>{
        &mut self.cab_requests
    }

    pub fn set_cab_requests(&mut self, orders: VecDeque<Order>) {
        self.cab_requests = orders;
    }

    pub fn add_cab_request(&mut self, order: Order) {
        self.cab_requests.push_back(order);
    }

    pub fn remove_cab_request(&mut self) {
        self.cab_requests.pop_front();
    }   
}

impl WorldView {
    pub fn new(my_elevator_id: u64, session_id: u64) -> Self{
        
        Self{
            my_elevator_id,
            session_id,
            elevator_statuses: Self::initialize_elevator_statuses(my_elevator_id),
            hall_order_queue: HashMap::new(),
            write_counter: Self::initialize_write_counter(my_elevator_id),
            heart_beat_counters: Self::initialize_heart_beat_counters(my_elevator_id),
            recorded_session_ids: Self::initialize_recorded_session_ids(my_elevator_id, session_id),
            }
    }

    //initializers

    fn initialize_elevator_statuses(id: u64) -> HashMap<u64, Elevator>{
        let mut initial_elevator_statuses = HashMap::new();
        initial_elevator_statuses.insert(id, Elevator::new(id));
        return initial_elevator_statuses
    }

    fn initialize_write_counter(id: u64) -> HashMap<u64, u64>{
        let mut initial_write_counter = HashMap::new();
        initial_write_counter.insert(id, 0 as u64);
        return initial_write_counter
    }

    fn initialize_heart_beat_counters(id: u64) -> HashMap<u64, u16>{
        let mut initial_heart_beat_counters: HashMap<u64, u16> = HashMap::new();
        initial_heart_beat_counters.insert(id, 0 as u16);
        return initial_heart_beat_counters
    }

    fn initialize_recorded_session_ids(elevator_id: u64, session_id: u64) -> HashMap<u64, u64>{
        let mut recorded_session_ids: HashMap<u64, u64> = HashMap::new();
        recorded_session_ids.insert(elevator_id, session_id);
        return recorded_session_ids;
    }

    //interface for elevators 
    fn get_id(&self) -> u64{
        return self.my_elevator_id
    }

    fn get_session_id(&self) -> u64{
        return self.session_id
    }

    pub fn get_elevator_statuses(&self) -> &HashMap<u64, Elevator>{
        return &self.elevator_statuses;
    }

    pub fn get_heart_beat_counters(&self) -> &HashMap<u64, u16>{
        return &self.heart_beat_counters
    }

    pub fn get_mut_heart_beat_counters(&mut self) -> &mut HashMap<u64, u16>{
        return &mut self.heart_beat_counters
    }

    pub fn get_heart_beat(&self, elevator_id: u64) -> &u16{
        match self.get_heart_beat_counters().get(&elevator_id){
            Some(heart_beat_counter) => return heart_beat_counter,
            None => return &0
        }
    }

    pub fn increment_heart_beat(&mut self, elevator_id: u64){
        match self.get_mut_heart_beat_counters().get_mut(&elevator_id){
            Some(heart_beat_counter) => {*heart_beat_counter += 1},
            None => {},
        } 
    }

    fn set_heart_beat(&mut self, elevator_id: u64, value: u16){
        match self.get_mut_heart_beat_counters().get_mut(&elevator_id){
            Some(heart_beat_counter) => {*heart_beat_counter = value},
            None => {},
        } 
    }

    fn get_recorded_session_ids(&self) -> &HashMap<u64, u64>{
        return &self.recorded_session_ids
    }

    fn get_mut_recorded_session_ids(&mut self) -> &mut HashMap<u64, u64>{
        return &mut self.recorded_session_ids
    }

    pub fn get_elevator(&self, elevator_id: u64) -> &Elevator {
        return self.elevator_statuses.get(&elevator_id)
        .expect(&format!("get error: no elevator found at {}.", elevator_id));
    }

    pub fn get_mut_elevator(&mut self, elevator_id: u64) -> &mut Elevator {
        return self.elevator_statuses.get_mut(&elevator_id)
        .expect(&format!("get_mut error: no elevator found at {}.", elevator_id));
    }

    pub fn set_elev_current_floor(&mut self, elevator_id: u64, floor: u8) {
        self.get_mut_elevator(elevator_id).set_floor(floor);
    }

    pub fn set_elev_direction(&mut self, elevator_id: u64, direction: ElevatorDirection) {
        self.get_mut_elevator(elevator_id).set_direction(direction);
    }

    pub fn get_elev_behaviour(&self, elevator_id: u64) -> &Behaviour{
        return &self.get_elevator(elevator_id).get_behaviour();
    }

    pub fn set_elev_behaviour(&mut self, elevator_id: u64, behaviour: Behaviour) {
        self.get_mut_elevator(elevator_id).set_behavior(behaviour);
    }

    pub fn get_elev_obstruction(&self, elevator_id: u64) -> &Obstruction{
        return &self.get_elevator(elevator_id).get_obstruction();
    }

    pub fn set_elev_obstruction(&mut self, elevator_id: u64, obstruction: Obstruction) {
        self.get_mut_elevator(elevator_id).set_obstruction(obstruction);
    }

    pub fn set_elev_cab_orders(&mut self, elevator_id: u64, orders: VecDeque<Order>) {
        self.get_mut_elevator(elevator_id).set_cab_requests(orders);
    }

    pub fn add_elev_cab_order(&mut self, elevator_id: u64, order: Order) {
        self.get_mut_elevator(elevator_id).add_cab_request(order);
    }

    pub fn remove_elev_cab_order(&mut self, elevator_id: u64) {
        self.get_mut_elevator(elevator_id).remove_cab_request();
    }

    //interface for order queue

    pub fn get_order_queue(&self) -> &HashMap<u64, Order>{
        return &self.hall_order_queue
    }

    fn get_mut_hall_order_queue(&mut self) -> &mut HashMap<u64, Order>{
        return &mut self.hall_order_queue
    }

    pub fn get_order(&self, order_id: u64) -> &Order{
        return self.hall_order_queue.get(&order_id)
        .expect(&format!("get error: no order found at {}.", order_id));
    }

    pub fn get_mut_order(&mut self, order_id: u64) -> Option<&mut Order>{
        return self.hall_order_queue.get_mut(&order_id)
    }

    pub fn add_to_queue(&mut self, order: Order) {
        self.hall_order_queue.insert(order.order_id, order);
    }

    pub fn remove_from_queue(&mut self, order_id: u64) {
        self.hall_order_queue.remove(&order_id);
    }

    pub fn set_order_status(&mut self, order_id: u64, status: OrderStatus) {
        match self.get_mut_order(order_id){
            Some(order) => {order.set_order_status(status)},
            None => {println!("could not set order status for order with ID: {}", order_id)},
        }
    }

    pub fn set_order_ack_barrier(&mut self, order_id: u64, barrier: Vec<u64>) {
        match self.get_mut_order(order_id){
            Some(order) => {order.set_ack_barrier(barrier)},
            None => {println!("could not set ack barrier for order with ID: {}", order_id)},
        }
    }

    pub fn insert_into_order_ack_barrier(&mut self, order_id: u64, elevator_id: u64) {
        match self.get_mut_order(order_id){
            Some(order) => {order.insert_into_ack_barrier(elevator_id)},
            None => {println!("could not insert into ack barrier for order with ID: {}", order_id)},
        }
    }

    fn order_status_manager(&mut self, hall_order_queue: &mut HashMap<u64, Order>){
        let mut orders_to_remove: Vec<u64> = Vec::new();
            for order in hall_order_queue.values_mut(){

                let order_id: u64 = order.get_order_id().clone();
                let unique_elevator_ids_count = order.get_ack_barrier().iter().collect::<HashSet<_>>().len();

                if (order.get_order_status() == &OrderStatus::Unconfirmed && unique_elevator_ids_count == 3){
                    order.set_order_status(OrderStatus::Confirmed);
                    order.get_mut_ack_barrier().clear();
                }

                else if (order.get_order_status() == &OrderStatus::Completed && unique_elevator_ids_count == 3){
                    order.set_order_status(OrderStatus::ReadyForDeletion);
                    order.get_mut_ack_barrier().clear();
                }

                else if (order.get_order_status() == &OrderStatus::ReadyForDeletion && unique_elevator_ids_count == 3){
                    orders_to_remove.push(order_id);
                }
            }

            for order_id in orders_to_remove{
                self.remove_from_queue(order_id);
            }
    }

    //to be kept here
    fn cab_order_status_manager(&mut self){
        let mut orders_to_remove: Vec<u64> = Vec::new();

        let my_cab_orders = self.get_mut_elevator(self.get_id()).get_mut_cab_requests();
            for order in my_cab_orders{
                let order_id: u64 = order.get_order_id().clone();
                let unique_elevator_ids_count = order.get_ack_barrier().iter().collect::<HashSet<_>>().len();

                if (order.get_order_status() == &OrderStatus::Unconfirmed && unique_elevator_ids_count == 3){
                    order.set_order_status(OrderStatus::Confirmed);
                    order.get_mut_ack_barrier().clear();
                }

                else if (order.get_order_status() == &OrderStatus::Completed && unique_elevator_ids_count == 3){
                    order.set_order_status(OrderStatus::ReadyForDeletion);
                    order.get_mut_ack_barrier().clear();
                }

                else if (order.get_order_status() == &OrderStatus::ReadyForDeletion && unique_elevator_ids_count == 3){
                    orders_to_remove.push(order_id);
                }
            }
            for order_id in orders_to_remove{
                self.remove_from_queue(order_id);
            }
    }
    //to be kept here
    fn update_my_hall_order_queue(&mut self, elevator_id: u64, world_view: &WorldView, me_resurrected: bool, other_resurrected: bool){

        if (self.is_counter_newer(&elevator_id, world_view, me_resurrected) == true) && !other_resurrected{

            let incoming_hall_order_queue: &HashMap<u64, Order> = world_view.get_order_queue();
            let my_id: u64 = self.get_id();

            let my_hall_order_queue: &mut HashMap<u64, Order> = self.get_mut_hall_order_queue();
                let incoming_hall_order_ids: Vec<&u64> = incoming_hall_order_queue.keys().collect();
                for &id in incoming_hall_order_ids{
                    let mut order: Order = world_view.get_order(id).clone(); 
                    if !my_hall_order_queue.keys().any(|k| *k == id){
                        order.insert_into_ack_barrier(my_id);
                        my_hall_order_queue.insert(id, order);
                    }
                }
        }
    } 

    //to be kept here
    fn update_my_write_counters(&mut self, world_view: &WorldView, me_resurrected: bool, other_resurrected: bool){

        let incoming_write_counters = world_view.get_write_counter();
        for elevator_id in incoming_write_counters.keys(){
            if (self.is_counter_newer(elevator_id, world_view, me_resurrected) == true) && !other_resurrected{
                self.set_write_counter(*elevator_id, *world_view.get_elevator_write_counter(*elevator_id));
            }
        }
    }

    //to be kept here
    fn update_my_heart_beat_counters(&mut self, elevator_id: u64, world_view: &WorldView, me_resurrected: bool, other_resurrected: bool){
        
        if (self.is_counter_newer(&elevator_id, world_view, me_resurrected) == true) && !other_resurrected{
            let incoming_heart_beat_counters: &HashMap<u64, u16> = world_view.get_heart_beat_counters();
            for id in incoming_heart_beat_counters.keys(){
                let value: &u16 = world_view.get_heart_beat(elevator_id);
                if *id != self.get_id(){
                    self.set_heart_beat(*id, *value);
                }
            }
        }
    }

    //to be kept here
    fn update_cab_ack_barriers(&mut self, elevator_id: u64, world_view: &WorldView, me_resurrected: bool, other_resurrected: bool){

        let my_id = self.get_id();

        if self.is_counter_newer(&elevator_id, world_view, me_resurrected) == true && !other_resurrected{

            if let Some(incoming_elevator) = world_view.elevator_statuses.get(&elevator_id){
                let incoming_cab_orders = incoming_elevator.get_cab_requests();
                for order in incoming_cab_orders{
                    if !order.get_ack_barrier().contains(&my_id){
                        let my_local_cab_requests = self.get_mut_elevator(elevator_id).get_mut_cab_requests();
                        if let Some(my_matching_order) = my_local_cab_requests.iter_mut().find(|o| o.get_order_id() == order.get_order_id()){
                            my_matching_order.insert_into_ack_barrier(my_id);
                        }
                        
                    }
                }
            }

            if let Some(my_elevator) = world_view.elevator_statuses.get(&my_id){
                let my_cab_requests = my_elevator.get_cab_requests();
                let my_local_elevator = self.get_mut_elevator(my_id);
                let my_local_cab_requests = my_local_elevator.get_mut_cab_requests();
                for order in my_cab_requests{
                    if let Some(my_matching_order) = my_local_cab_requests.iter_mut().find(|o| o.get_order_id() == order.get_order_id()){
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

    fn update_hall_ack_barriers(&mut self, elevator_id: u64, world_view: &WorldView, me_resurrected: bool, other_resurrected: bool){

        if (self.is_counter_newer(&elevator_id, world_view, me_resurrected) == true) && !other_resurrected{

            let incoming_hall_order_queue: &HashMap<u64, Order> = world_view.get_order_queue();

            for order in incoming_hall_order_queue.values(){ 

                    let order_id: u64 = order.get_order_id().clone();
                    let elevator_ids_in_other_barrier: Vec<u64> = order.get_ack_barrier().iter().cloned().collect();

                    match self.get_mut_order(order_id){
                        Some(my_order) 
                        => {for id in elevator_ids_in_other_barrier{
                                if !my_order.get_ack_barrier().contains(&id){my_order.insert_into_ack_barrier(id);}}},
                        None 
                        //do nothing
                        => {},
                    }
                }
        }
    }
    //write counter incrementer

    fn get_write_counter(&self) -> &HashMap<u64, u64>{
        return &self.write_counter
    }

    fn get_elevator_write_counter(&self, elevator_id: u64) -> &u64{
        match self.get_write_counter().get(&elevator_id){
            Some(write_counter) => return write_counter,
            None => return &0
        }
    }

    fn get_mut_write_counter(&mut self) -> &mut HashMap<u64, u64>{
        return &mut self.write_counter
    }

    fn set_write_counter(&mut self, elevator_id: u64, value: u64) {
        match self.get_mut_write_counter().get_mut(&elevator_id){
            Some(counter) => *counter = value,
            None => {},
        }
    }
 
    pub fn increment_write_counter(&mut self, elevator_id: &u64){
        let counter = self.write_counter.entry(*elevator_id).or_insert(0);
        *counter = counter.wrapping_add(1);
    }

    fn get_unknown_counter(&mut self, world_view: &WorldView, elevetor_id: u64){
        
        if !self.get_write_counter().keys().any(|k| k == &elevetor_id){
                let incoming_write_counter: &u64 = match world_view.get_write_counter().get(&elevetor_id){
                    Some(counter) => counter,
                    None => &0,
                };
            self.get_mut_write_counter().insert(elevetor_id, *incoming_write_counter);
        }
    }

    //synchronizing functions

    fn is_counter_newer(&mut self, elevator_id: &u64, world_view: &WorldView, me_resurrected: bool) -> bool{

        let incoming_write_counter: &u64 = match world_view.get_write_counter().get(&elevator_id){
                Some(counter) => counter,
                None => return false,
            };

        let my_last_recorded_counter: &mut u64 = self.get_mut_write_counter().entry(*elevator_id).or_insert(0); 
        let version_control = incoming_write_counter.wrapping_sub(*my_last_recorded_counter);
        
        let mut is_newer: bool = false;
        match me_resurrected{
            false => {if version_control != 0 && version_control < (1 << 63){is_newer = true}}

            true => {if version_control < (1 << 63){is_newer = true}}  
        }  

        return is_newer
    }

    fn synchronize_world_view(&mut self, world_view: WorldView) {

        let my_id: u64 = self.get_id();
        let my_session_id:u64 = self.get_session_id();

        let other_id: u64 = world_view.get_id();
        let other_session_id: u64 = world_view.get_session_id();

        let last_recorded_session_ids: &mut HashMap<u64, u64> = self.get_mut_recorded_session_ids();
        let other_last_recorded_session_ids: &HashMap<u64, u64> = world_view.get_recorded_session_ids();

        let mut other_resurrected: bool = false;
        let mut me_resurrected: bool = false;

        if !last_recorded_session_ids.keys().any(|k| *k == other_id){
            last_recorded_session_ids.insert(other_id, other_session_id);
        } 

        if last_recorded_session_ids.get(&other_id).expect("critical error synchronizing session ids.") != &other_session_id{
            last_recorded_session_ids.insert(other_id, other_session_id);
            other_resurrected = true;
        }

        if let Some(session_id) = other_last_recorded_session_ids.get(&my_id){
            if session_id != &my_session_id{me_resurrected = true};
        }

        let other_elevators: Vec<&Elevator> = world_view.get_elevator_statuses()
                                                        .values()
                                                        .collect();

        for id in other_last_recorded_session_ids.keys(){

            self.get_unknown_counter(&world_view, *id);

            let mut is_newer: bool = false;
            is_newer = self.is_counter_newer(id, &world_view, me_resurrected);

            if is_newer && !other_resurrected{
                if !self.elevator_statuses.contains_key(id){
                    self.elevator_statuses.insert(*id, world_view.get_elevator(*id).clone());
                }

                let my_stored_elevator: &mut Elevator = self.get_mut_elevator(id.clone()); //what if not yet stored?
                let other_stored_elevator: &Elevator = world_view.get_elevator(id.clone());

                my_stored_elevator.set_behavior(*other_stored_elevator.get_behaviour()); 
                my_stored_elevator.set_obstruction(*other_stored_elevator.get_obstruction()); 
                my_stored_elevator.set_floor(*other_stored_elevator.get_floor()); 
                my_stored_elevator.set_direction(*other_stored_elevator.get_direction());
                my_stored_elevator.set_cab_requests(*other_stored_elevator.get_cab_requests());
                my_stored_elevator.set_dead_or_alive(*other_stored_elevator.get_dead_or_alive());
            }
        }

        let incoming_hall_order_queue: &HashMap<u64, Order> = world_view.get_order_queue();
        update_my_hall_order_queue(other_id, world_view, me_resurrected, other_resurrected);

        update_hall_ack_barriers(incoming_hall_order_queue);
        
        self.order_status_manager(my_hall_order_queue);
        self.cab_order_status_manager();

        self.update_cab_ack_barriers(&world_view, me_resurrected, other_resurrected);
        self.update_my_heart_beat_counters(other_id, &world_view, me_resurrected, other_resurrected);
        self.update_my_write_counters(&world_view, me_resurrected, other_resurrected);
        self.increment_write_counter(&my_id);

        for id in self.recorded_session_ids.keys(){
            if (*id != my_id && *id != other_id){
                self.increment_heart_beat(*id);
            }
            if self.get_heart_beat(*id) >= &300{
                self.get_mut_elevator(*id).set_dead_or_alive(DeadOrAlive::Dead);
            }
        }

        self.set_heart_beat(other_id, 0);
        if self.get_elevator(other_id).get_dead_or_alive() == &DeadOrAlive::Dead{
            self.get_mut_elevator(other_id).set_dead_or_alive(DeadOrAlive::Alive);
        }


    }      
    

    //editing functions 

    pub fn edit_elevator_status(&mut self, command: ElevatorStatusCommand) {
        match command { 
            ElevatorStatusCommand::SetFloor {elevator_id, floor} 
            => self.set_elev_current_floor(elevator_id, floor),

            ElevatorStatusCommand::SetDirection {elevator_id, dir} 
            => self.set_elev_direction(elevator_id, dir),

            ElevatorStatusCommand::SetObstruction {elevator_id, obstruction} 
            => self.set_elev_obstruction(elevator_id, obstruction),

            ElevatorStatusCommand::SetBehaviour {elevator_id, behavior} 
            => self.set_elev_behaviour(elevator_id, behavior),

            ElevatorStatusCommand::SetCabRequests {elevator_id, orders} 
            => self.set_elev_cab_orders(elevator_id, orders),

            ElevatorStatusCommand::AddCabRequest {elevator_id, order}
            =>self.add_elev_cab_order(elevator_id, order),

            ElevatorStatusCommand::RemoveCabRequest {elevator_id}
            =>self.remove_elev_cab_order(elevator_id),

            ElevatorStatusCommand::SynchronizeWorldView {world_view}
            =>self.synchronize_world_view(world_view),

        }
    }

    pub fn edit_order_queue(&mut self, command: OrderQueueCommand) {
        match command {
            OrderQueueCommand::AddToOrderQueue {order}
            => self.add_to_queue(order),

            OrderQueueCommand::RemoveFromOrderQueue {order_id}
            => self.remove_from_queue(order_id),

            OrderQueueCommand::SetOrderStatus {order_id, status}
            =>self.set_order_status(order_id, status),

            OrderQueueCommand::SetAckBarrier {order_id, barrier}
            => self.set_order_ack_barrier(order_id, barrier),

            OrderQueueCommand::InsertAckBarrier {order_id, elevator_id}
            => self.insert_into_order_ack_barrier(order_id, elevator_id),
        }
    }


}





