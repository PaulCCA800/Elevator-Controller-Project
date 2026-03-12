use std::convert::TryFrom;
use serde::{Deserialize, Serialize};

use crate::{mem::{Direction, ElevatorStatusCommand, Order, OrderStatus}, message::{LOCAL_ID, hardware_msg::{HardwareData}, network_msg::NetworkData}};

fn is_cab(call: u8) -> bool {
    call == 2
}

fn get_dir(call: u8) -> Direction {
    match call {
        2 => Direction::Inherit,
        1 => Direction::Down,
        _ => Direction::Up,
    }
}

#[derive(Serialize, Deserialize)]
pub struct MemoryData {
    pub data: ElevatorStatusCommand,
}

impl TryFrom<HardwareData> for MemoryData {
    type Error = ();

    fn try_from(data: HardwareData) -> Result<Self, Self::Error> {
        match data {
            HardwareData::CallButton(call_button) => Ok(Self {
                data: ElevatorStatusCommand::AddOrder { 
                elevator_id: LOCAL_ID, 
                order: Order::new(
                    LOCAL_ID,
                    call_button.floor, 
                    is_cab(call_button.call), 
                    get_dir(call_button.call),
                    OrderStatus::Unconfirmed, 
                    vec![], 
                    LOCAL_ID
                )}
            }),
            HardwareData::FloorSensor(floor) => Ok(Self {
                data: ElevatorStatusCommand::SetFloor{
                    elevator_id: LOCAL_ID,
                    floor 
                }
            }),
            HardwareData::Obstruction(status) => Ok(Self {
                data: ElevatorStatusCommand::SetObstruction { 
                    elevator_id: LOCAL_ID, 
                    obs: status 
                }
            }),
            HardwareData::StopButton(status) => Ok(Self {
                data: ElevatorStatusCommand::SetStop { 
                    elevator_id: LOCAL_ID, 
                    stop: status 
                }
            }), 
            _ => Err(())
        }
    }
}

impl From<NetworkData> for MemoryData {
    fn from(data: NetworkData) -> Self {
        Self {
            data: ElevatorStatusCommand::AddWorldView { 
                elevator_id: data.machine_id, 
                world: data.data 
            }   
        }
    }
}