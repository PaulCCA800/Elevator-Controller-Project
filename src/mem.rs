use std::{collections::{HashMap, VecDeque}};
use serde::{Deserialize, Serialize};
use rand::Rng;

use crate::misc::generate_id;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Behaviour {
    Idle,
    Moving,
    DoorOpen,
}

#[derive(Clone)]
pub enum OrderType {
    Cab,
    Hall,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElevatorDirection {
    Up,
    Down,
    Stop,
}

#[derive(Clone, Debug)]
pub enum OrderDirection {
    Up,
    Down
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OrderStatus {
    Unconfirmed,
    Confirmed,
    Completed, 
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
    elevator_id: u64,
    behaviour: Behaviour,
    floor: u8,
    direction: ElevatorDirection,
    cab_requests: VecDeque<Order>,
}

#[derive(Clone)]
pub struct WorldView {
    elevator_statuses: HashMap <u64, Elevator>,
    hall_order_queue: VecDeque<Order>,
    write_counter: HashMap <u64, u8>,
}

pub enum ElevatorStatusCommand {
    SetFloor {elevator_id: u64, floor: u8},
    SetDirection {elevator_id: u64, dir: Direction},
    SetObstruction {elevator_id: u64, obs: bool},
    SetStop {elevator_id: u64, stop: bool},
    SetCabOrders {elevator_id: u64, orders: VecDeque<Order>},
    SetHallOrders {elevator_id: u64, orders: VecDeque<Order>},
    AddCabOrder{elevator_id: u64, order: Order},
    RemoveCabOrder {elevator_id: u64},
    AddHallOrder{elevator_id: u64, order: Order},
    RemoveHallOrder {elevator_id: u64},
}

pub enum OrderQueueCommand {
    AddToOrderQueue {order: Order},
    RemoveFromOrderQueue{order_id: u64},
    SetOrderStatus{order_id: u64, status: OrderStatus},
    SetAckBarrier{order_id: u64, barrier: Vec<u64>},
    InsertAckBarrier{order_id: u64, elevator_id: u64},
    AssignOrder{order_id: u64, elevator_id: u64},
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
    pub fn new(elevator_id: u64, floor: u8) -> Self{
        Self{
            elevator_id,
            behaviour: Behaviour::Idle,
            floor,
            direction: ElevatorDirection::Stop,
            cab_requests: Self::initialize_cab_requests(),
        }
    }

    fn initialize_cab_requests() -> VecDeque<Order>{
        return VecDeque::new();
    }

    pub fn get_elevator_id(&self) -> &u64{
        return &self.elevator_id
    }

    pub fn get_behaviour(&self) -> &Behaviour{
        return &self.behaviour
    }

    pub fn get_floor(&self) -> &u8{
        return &self.floor
    }

    pub fn set_current_floor(&mut self, floor: u8) {
        self.floor = floor;
    }

    pub fn get_direction(&mut self) -> &Direction {
        return &self.direction
    }

    pub fn set_direction(&mut self, dir: Direction) {
        self.direction = dir;
    }

    pub fn get_cab_requests(&self) -> &VecDeque<Order>{
        &self.cab_requests
    }

    pub fn set_cab_requests(&mut self, orders: VecDeque<Order>) {
        self.cab_requests = orders;
    }

    pub fn add_cab_order(&mut self, order: Order) {
        self.cab_requests.push_back(order);
    }

    pub fn remove_cab_order(&mut self) {
        self.cab_requests.pop_front();
    }   
}

impl WorldView {
    pub fn new(my_elevator_id: u64) -> Self{
        
        Self{
            elevator_statuses: Self::initialize_elevator_statuses(my_elevator_id),
            hall_order_queue: VecDeque::new(),
            write_counter: Self::initialize_write_counter(my_elevator_id),
            }
    }

    //initializers

    fn initialize_elevator_statuses(id: u64) -> HashMap<u64, Elevator>{
        let mut initial_elevator_statuses = HashMap::new();
        initial_elevator_statuses.insert(id, Elevator::new(id, 1));
        return initial_elevator_statuses
    }

    fn initialize_write_counter(id: u64) -> HashMap<u64, u8>{
        let mut initial_write_counter = HashMap::new();
        initial_write_counter.insert(id, 0 as u8);
        return initial_write_counter
    }

    //interface for elevators 

    pub fn get_elevator_statuses(&self) -> &HashMap<u64, Elevator>{
        return &self.elevator_statuses
    }

    pub fn get_elevator(&self, elevator_id: u64) -> &Elevator {
        self.elevator_statuses.get(&elevator_id).expect(&format!("get error: no elevator found at {}.", elevator_id))
    }

    pub fn get_mut_elevator(&mut self, elevator_id: u64) -> &mut Elevator {
        self.elevator_statuses.get_mut(&elevator_id).expect(&format!("get_mut error: no elevator found at {}.", elevator_id))
    }

    pub fn set_elev_current_floor(&mut self, elevator_id: u64, floor: u8) {
        self.get_mut_elevator(elevator_id).set_current_floor(floor);
    }

    pub fn set_elev_direction(&mut self, elevator_id: u64, direction: Direction) {
        self.get_mut_elevator(elevator_id).set_direction(direction);
    }

    pub fn set_elev_stop(&mut self, elevator_id: u64, stop: bool) {
        self.get_mut_elevator(elevator_id).set_stop(stop);
    }

    pub fn set_elev_cab_orders(&mut self, elevator_id: u64, orders: VecDeque<Order>) {
        self.get_mut_elevator(elevator_id).set_cab_orders(orders);
    }

    pub fn set_elev_hall_orders(&mut self, elevator_id: u64, orders: VecDeque<Order>) {
        self.get_mut_elevator(elevator_id).set_hall_orders(orders);
    }

    pub fn add_elev_cab_order(&mut self, elevator_id: u64, order: Order) {
        self.get_mut_elevator(elevator_id).add_cab_order(order);
    }

    pub fn remove_elev_cab_order(&mut self, elevator_id: u64) {
        self.get_mut_elevator(elevator_id).remove_cab_order();
    }

    pub fn add_elev_hall_order(&mut self, elevator_id: u64, order: Order) {
        self.get_mut_elevator(elevator_id).add_hall_order(order);
    }

    pub fn remove_elev_hall_order(&mut self, elevator_id: u64) {
        self.get_mut_elevator(elevator_id).remove_hall_order();
    }

    //interface for order queue

    pub fn get_order_queue(&mut self) -> &VecDeque<Order>{
        return &self.hall_order_queue
    }

    pub fn get_order(&self, order_id: u64) -> &Order{
        self.orderQueue.get(&order_id).expect(&format!("get error: no order found at {}.", order_id))
    }

    pub fn get_mut_order(&mut self, order_id: u64) -> &mut Order{
        self.orderQueue.get_mut(&order_id).expect(&format!("get_mut error: no order found at {}.", order_id))
    }

    pub fn add_to_queue(&mut self, order: Order) {
        self.orderQueue.insert(order.order_id, order);
    }

    pub fn remove_from_queue(&mut self, order_id: u64) {
        self.orderQueue.remove(&order_id);
    }

    pub fn set_order_status(&mut self, order_id: u64, status: OrderStatus) {
        self.get_mut_order(order_id).set_status(status);
    }

    pub fn set_order_ack_barrier(&mut self, order_id: u64, barrier: Vec<u64>) {
        self.get_mut_order(order_id).set_ack_barrier(barrier);
    }

    pub fn insert_into_order_ack_barrier(&mut self, order_id: u64, elevator_id: u64) {
        self.get_mut_order(order_id).insert_into_ack_barrier(elevator_id);
    }

    pub fn assign_order_to_elevator(&mut self, order_id: u64, elevator_id: u64) {
        self.get_mut_order(order_id).assign_to_elevator(elevator_id);
    }

    //editing functions 

    pub fn edit_elevator_status(&mut self, command: ElevatorStatusCommand) {
        match command { 
            ElevatorStatusCommand::SetFloor {elevator_id, floor} 
            => self.set_elev_current_floor(elevator_id, floor),

            ElevatorStatusCommand::SetDirection {elevator_id, dir} 
            => self.set_elev_direction(elevator_id, dir),

            ElevatorStatusCommand::SetObstruction {elevator_id, obs} 
            => self.set_elev_obstruction(elevator_id, obs),

            ElevatorStatusCommand::SetStop {elevator_id, stop} 
            => self.set_elev_stop(elevator_id, stop),

            ElevatorStatusCommand::SetCabOrders {elevator_id, orders} 
            => self.set_elev_cab_orders(elevator_id, orders),

            ElevatorStatusCommand::SetHallOrders {elevator_id, orders} 
            => self.set_elev_hall_orders(elevator_id, orders), 

            ElevatorStatusCommand::AddCabOrder {elevator_id, order}
            =>self.add_elev_cab_order(elevator_id, order),

            ElevatorStatusCommand::RemoveCabOrder {elevator_id}
            =>self.remove_elev_cab_order(elevator_id),

            ElevatorStatusCommand::AddHallOrder {elevator_id, order}
            =>self.add_elev_hall_order(elevator_id, order),

            ElevatorStatusCommand::RemoveHallOrder {elevator_id}
            =>self.remove_elev_hall_order(elevator_id),
   

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

            OrderQueueCommand::AssignOrder {order_id, elevator_id}
            => self.assign_order_to_elevator(order_id, elevator_id),
        }
    }

    pub fn increment_write_counter(&mut self, elevator_id: &u64) {
        *self.writeCounter.entry(*elevator_id).or_insert(0) += 1;
    }

}





