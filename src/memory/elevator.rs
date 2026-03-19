use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use crate::memory::order::Order;
use crate::memory::world_view::WorldView;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Behaviour {
    Idle,
    Moving,
    DoorOpen,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Elevator {
    elevator_id: u64,
    session_id: u64,
    behaviour: Behaviour,
    obstruction: Obstruction,
    floor: u8,
    direction: ElevatorDirection,
    cab_requests: VecDeque<Order>,
}

#[derive(Serialize, Deserialize)]
pub enum ElevatorStatusCommand {
    SetBehaviour        {elevator_id: u64, behavior: Behaviour},
    SetObstruction      {elevator_id: u64, obstruction: Obstruction},
    SetFloor            {elevator_id: u64, floor: u8},
    SetDirection        {elevator_id: u64, dir: ElevatorDirection},
    SetCabRequests      {elevator_id: u64, orders: VecDeque<Order>},
    AddCabRequest       {elevator_id: u64, order: Order},
    RemoveCabRequest    {elevator_id: u64},
    SynchronizeWorldView{elevator_id: u64, world_view: WorldView},
}

impl Elevator{
    pub fn new(elevator_id: u64) -> Self{
        Self{
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
