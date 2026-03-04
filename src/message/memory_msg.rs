use crate::mem::{Direction, ElevatorStatusCommand, Order, OrderStatus};
use driver_rust::elevio::poll::CallButton;

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
        return call == 2;
    }

    fn
    get_dir(call: u8) -> Direction
    {
        match call
        {
            2 => return Direction::Inherit,
            1 => return Direction::Down,
            _ => return Direction::Up,
        }
    }
}

// Network Functions
impl
MemoryData
{
    pub fn
    from_network_data()
    {

    }
}

// Hardware Functions
impl
MemoryData
{
    pub fn
    from_call_button(call_button_data: CallButton, id: u64) -> Self
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