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
    session_id: u64,
    behaviour: Behaviour,
    obstruction: Obstruction,
    floor: u8,
    direction: ElevatorDirection,
    cab_requests: VecDeque<Order>,
}

#[derive(Clone)]
pub struct WorldView {
    elevator_statuses: HashMap <u64, Elevator>,
    hall_order_queue: HashMap<u64, Order>,
    write_counter: HashMap <u64, u8>,
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
    pub fn new() -> Self{
        Self{
            elevator_id: generate_id(),
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

    pub fn get_direction(&mut self) -> &ElevatorDirection {
        return &self.direction
    }

    pub fn set_direction(&mut self, dir: ElevatorDirection) {
        self.direction = dir;
    }

    pub fn get_cab_requests(&self) -> &VecDeque<Order>{
        &self.cab_requests
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
    pub fn new(my_elevator_id: u64) -> Self{
        
        Self{
            elevator_statuses: Self::initialize_elevator_statuses(my_elevator_id),
            hall_order_queue: HashMap::new(),
            write_counter: Self::initialize_write_counter(my_elevator_id),
            }
    }

    //initializers

    fn initialize_elevator_statuses(id: u64) -> HashMap<u64, Elevator>{
        let mut initial_elevator_statuses = HashMap::new();
        initial_elevator_statuses.insert(id, Elevator::new());
        return initial_elevator_statuses
    }

    fn initialize_write_counter(id: u64) -> HashMap<u64, u8>{
        let mut initial_write_counter = HashMap::new();
        initial_write_counter.insert(id, 0 as u8);
        return initial_write_counter
    }

    //interface for elevators 

    pub fn get_elevator_statuses(&self) -> &HashMap<u64, Elevator>{
        return &self.elevator_statuses;
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

    pub fn get_order_queue(&mut self) -> &HashMap<u64, Order>{
        return &self.hall_order_queue
    }

    pub fn get_order(&self, order_id: u64) -> &Order{
        return self.hall_order_queue.get(&order_id)
        .expect(&format!("get error: no order found at {}.", order_id));
    }

    pub fn get_mut_order(&mut self, order_id: u64) -> &mut Order{
        return self.hall_order_queue.get_mut(&order_id)
        .expect(&format!("get_mut error: no order found at {}.", order_id));
    }

    pub fn add_to_queue(&mut self, order: Order) {
        self.hall_order_queue.insert(order.order_id, order);
    }

    pub fn remove_from_queue(&mut self, order_id: u64) {
        self.hall_order_queue.remove(&order_id);
    }

    pub fn set_order_status(&mut self, order_id: u64, status: OrderStatus) {
        self.get_mut_order(order_id).set_order_status(status);
    }

    pub fn set_order_ack_barrier(&mut self, order_id: u64, barrier: Vec<u64>) {
        self.get_mut_order(order_id).set_ack_barrier(barrier);
    }

    pub fn insert_into_order_ack_barrier(&mut self, order_id: u64, elevator_id: u64) {
        self.get_mut_order(order_id).insert_into_ack_barrier(elevator_id);
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

    pub fn increment_write_counter(&mut self, elevator_id: &u64) {
        *self.write_counter.entry(*elevator_id).or_insert(0) += 1;
    }

    pub fn update_my_world_view(){

    }

    pub fn 

}





