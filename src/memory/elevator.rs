use std::collections::VecDeque;
use serde::{Serialize, Deserialize};

use crate::memory::order::Order;


#[derive(Serialize, Deserialize, Copy,Clone,Eq, PartialEq, Debug)]
pub enum DeadOrAlive {
    Dead,
    Alive,
}


#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Behaviour {
    Idle,
    Moving,
    DoorOpen,
}


#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Obstruction {
    Obstructed,
    Clear,
}


#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ElevatorDirection {
    Up,
    Down,
    Stop,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Elevator {
    dead_or_alive: DeadOrAlive,
    elevator_id: u16,
    session_id: u16,
    behaviour: Behaviour,
    obstruction: Obstruction,
    floor: u8,
    direction: ElevatorDirection,
    cab_orders: VecDeque<Order>,
}


impl Elevator{
    
    pub fn new(elevator_id: u16, session_id: u16) -> Self {
        Self{
            dead_or_alive: DeadOrAlive::Alive,
            elevator_id,
            session_id,
            behaviour: Behaviour::Idle,
            obstruction: Obstruction::Clear,
            floor: 1,
            direction: ElevatorDirection::Stop,
            cab_orders: Self::initialize_cab_orders(),
        }
    }


    fn initialize_cab_orders() -> VecDeque<Order> {
        return VecDeque::new();
    }


    pub fn get_dead_or_alive(&self) -> &DeadOrAlive {
        return &self.dead_or_alive
    }


    pub fn set_dead_or_alive(&mut self, status: DeadOrAlive) {
        self.dead_or_alive = status
    }


    pub fn get_elevator_id(&self) -> &u16 {
        return &self.elevator_id
    }


    pub fn get_session_id(&self) -> &u16 {
        return &self.session_id
    }


    pub fn get_behaviour(&self) -> &Behaviour {
        return &self.behaviour
    }


    pub fn set_behavior(&mut self, behaviour: Behaviour) {
        self.behaviour = behaviour
    }


    pub fn get_obstruction(&self) -> &Obstruction {
        return &self.obstruction
    }


    pub fn set_obstruction(&mut self, obstruction: Obstruction) {
        self.obstruction = obstruction
    }


    pub fn get_floor(&self) -> &u8 {
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


    pub fn get_cab_orders(&self) -> &VecDeque<Order> {
        &self.cab_orders
    }


    pub fn get_mut_cab_orders(&mut self) -> &mut VecDeque<Order> {
        &mut self.cab_orders
    }


    pub fn set_cab_orders(&mut self, orders: VecDeque<Order>) {
        self.cab_orders = orders;
    }


    pub fn add_cab_order(&mut self, order: Order) {
        self.cab_orders.push_back(order);
    }


    pub fn remove_cab_order(&mut self) {
        self.cab_orders.pop_front();
    }   
}
