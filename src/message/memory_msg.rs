use std::convert::TryFrom;
use serde::{Deserialize, Serialize};

use crate::{mem::{Direction, ElevatorStatusCommand, Order, OrderStatus, WorldView}, message::{LOCAL_ID, hardware_msg::{ConvertedCallButton, HardwareData}, network_msg::NetworkData}};

#[derive(Serialize, Deserialize)]
pub struct
MemoryData
{
    pub data: ElevatorStatusCommand,
}

// "Global" Functions
impl 
MemoryData
{
    fn
    is_cab(call: u8) -> bool
    {
        call == 2
    }

    fn
    get_dir(call: u8) -> Direction
    {
        match call
        {
            2 => Direction::Inherit,
            1 => Direction::Down,
            _ => Direction::Up,
        }
    }
}

impl TryFrom<HardwareData> for MemoryData {
    type Error = ();

    fn try_from(data: HardwareData) -> Result<Self, Self::Error> {
        match data
        {
            HardwareData::CallButton(call_button) 
                => Ok(MemoryData::from_call_button(call_button, LOCAL_ID)),
            HardwareData::FloorSensor(floor) 
                => Ok(MemoryData::from_floor(floor, LOCAL_ID)),
            HardwareData::Obstruction(status)
                => Ok(MemoryData::from_obstruction(status, LOCAL_ID)),
            HardwareData::StopButton(status)
                => Ok(MemoryData::from_stop_button(status, LOCAL_ID)), 
            _ => Err(())
        }
    }
}

impl From<NetworkData> for MemoryData {
    fn from(data: NetworkData) -> Self {
        MemoryData::from_network_data(data.data, data.machine_id)
    }
}

// Network Functions
impl
MemoryData
{
    pub fn
    from_network_data(data: WorldView, id: u64) -> Self
    {
        MemoryData { 
            data: ElevatorStatusCommand::AddWorldView { 
                elevator_id: id, 
                world: data 
            }
        }
    }
}

// Hardware Functions
impl
MemoryData
{
    pub fn
    from_call_button(call_button_data: ConvertedCallButton, id: u64) -> Self
    {
        MemoryData
        {
            data: ElevatorStatusCommand::AddOrder
            { 
                elevator_id: id, 
                order: Order::new(id,
                    call_button_data.floor, 
                    MemoryData::is_cab(call_button_data.call), 
                    MemoryData::get_dir(call_button_data.call),
                    OrderStatus::Unconfirmed, 
                    vec![], 
                    id)
            }
        }
    }

    pub fn
    from_floor(floor: u8, id: u64) -> Self
    {
        MemoryData
        {
            data: ElevatorStatusCommand::SetFloor{
                elevator_id: id,
                floor 
            }
        }
    }

    pub fn
    from_obstruction(status: bool, id: u64) -> Self
    {
        MemoryData
        {
            data: ElevatorStatusCommand::SetObstruction { 
                elevator_id: id, 
                obs: status 
            }
        }
    }

    pub fn
    from_stop_button(status: bool, id: u64) -> Self
    {
        MemoryData
        {
            data: ElevatorStatusCommand::SetStop { 
                elevator_id: id, 
                stop: status 
            }
        }
    }
}