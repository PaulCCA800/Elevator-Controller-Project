use crate::mem::{ElevatorStatusCommand, Order};
use driver_rust::elevio::poll::CallButton;

pub struct
MemoryData
{
    data: ElevatorStatusCommand,
}

impl
MemoryData
{
    pub fn
    from_call_button() -> Self
    {
        MemoryData
        {
            data: ElevatorStatusCommand::AddOrder
            { 
                elevator_id: (), 
                order: () 
            }
        }
    }
}