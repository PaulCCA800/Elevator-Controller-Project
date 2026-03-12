use std::{convert::TryFrom, os::linux::raw::stat};
use serde::{Deserialize, Serialize};

use crate::{memory::{ElevatorDirection, ElevatorStatusCommand}, message::{LOCAL_ID, hardware_msg::HardwareData, network_msg::NetworkData}};

fn is_cab(call: u8) -> bool {
    call == 2
}

fn get_dir(call: u8) -> ElevatorDirection {
    match call {
        2 => ElevatorDirection::Stop,
        1 => ElevatorDirection::Down,
        _ => ElevatorDirection::Up,
    }
}

fn get_obstruction(obs: bool) -> Obstruction
{
    match obs {
        true => Obstruction::Obstructed,
        false => Obstruction::Clear
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
            /*
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
             */
            HardwareData::FloorSensor(floor) => Ok(Self {
                data: ElevatorStatusCommand::SetFloor{
                    elevator_id: LOCAL_ID,
                    floor 
                }
            }),

            HardwareData::Obstruction(status) => Ok(Self {
                data: ElevatorStatusCommand::SetObstruction { 
                    elevator_id: LOCAL_ID, 
                    obstruction: get_obstruction(status) 
                }
            }),
            /*HardwareData::StopButton(status) => Ok(Self {
                data: ElevatorStatusCommand::SetStop { 
                    elevator_id: LOCAL_ID, 
                    stop: status 
                }
            }),
            */ 
            _ => Err(())
        }
    }
}

impl From<NetworkData> for MemoryData {
    fn from(data: NetworkData) -> Self {
        Self {
            data: ElevatorStatusCommand::SynchronizeWorldView {
                world_view: data.data
            }
        }
    }
}