use serde::{Deserialize, Serialize};

use std::collections::VecDeque;
use crate::memory::{Order, WorldView};

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
    SetBehaviour {elevator_id: u64, behavior: Behaviour},
    SetObstruction {elevator_id: u64, obstruction: Obstruction},
    SetFloor {elevator_id: u64, floor: u8},
    SetDirection {elevator_id: u64, dir: ElevatorDirection},
    SetCabRequests {elevator_id: u64, orders: VecDeque<Order>},
    AddCabRequest {elevator_id: u64, order: Order},
    RemoveCabRequest {elevator_id: u64},
    SynchronizeWorldView {world_view: WorldView},
}