use std::convert::TryFrom;
use driver_rust::elevio::poll::CallButton;
use serde::{Deserialize, Serialize};

use crate::message::memory_msg::MemoryData;
use crate::memory::ElevatorStatusCommand;

#[derive(Serialize, Deserialize)]
pub struct
ConvertedCallButton
{
    pub floor: u8,
    pub call: u8,
}

impl 
ConvertedCallButton 
{
    pub fn
    from_call_button(call_button: CallButton) -> Self
    {
        Self{
            floor: call_button.floor,
            call: call_button.call,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum
HardwareData
{
    CallButton(ConvertedCallButton),
    FloorSensor(u8),
    StopButton(bool),
    Obstruction(bool),
    SetCallButtonLight{floor: u8, call: u8, status: bool},
    SetMotorDirection(u8),
    SetFloorIndicator(u8),
    SetDoorLight(bool),
    SetStopLight(bool),
}

impl TryFrom<MemoryData> for HardwareData {
    type Error = ();

    fn try_from(data: MemoryData) -> Result<Self, Self::Error> {
        match data.data {
            //ElevatorStatusCommand::SetDirection { elevator_id: _, dir } => Ok(
            //    HardwareData::SetMotorDirection(dir.to_u8())
            //),
            ElevatorStatusCommand::SetFloor { elevator_id: _, floor } => Ok(
                HardwareData::SetFloorIndicator(floor)
            ),
            //ElevatorStatusCommand::SetStop { elevator_id: _, stop } => Ok(
            //    HardwareData::SetStopLight(stop)
            //),
            _ => Err(())
        }
    }
}
