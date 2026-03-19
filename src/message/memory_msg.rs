use std::convert::TryFrom;
use serde::{Deserialize, Serialize};

use crate::{message::{LOCAL_ID, hardware_msg::HardwareData, network_msg::NetworkData}};
use crate::memory::elevator::{Obstruction};
use crate::memory::world_view::ElevatorStatusCommand;
use crate::memory::orders::{Order, OrderDirection, OrderType};

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
            
            HardwareData::CallButton(call_button) => Ok(Self {
                data: ElevatorStatusCommand::AddCabRequest { 
                    elevator_id: LOCAL_ID, 
                    order: Order::new(
                        call_button.floor, 
                        OrderType::is_cab(call_button.call),
                        OrderDirection::dir_from_call(call_button.call)
                    ) 
                }
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