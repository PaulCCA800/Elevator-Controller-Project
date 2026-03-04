use std::fmt::Error;

use crate::mem::ElevatorStatusCommand;

use crate::message::{
    hardware_msg::HardwareData, 
    network_msg::NetworkData,
    memory_msg::MemoryData
};

impl
HardwareData
{
    pub fn
    hardware_to_memory(self, id: u64) -> Result<MemoryData, Error>
    {
        match self
        {
            Self::DataCallButton { call_button_data } 
                => Ok(MemoryData::from_call_button(call_button_data, id)),
            Self::DataFloorSensor { floor } 
                => Ok(MemoryData::from_floor(floor, id)),
            Self::DataObstruction { status }  
                => Ok(MemoryData::from_obstruction(status, id)),
            Self::DataStopButton { status } 
                => Ok(MemoryData::from_stop_button(status, id)),
            _ => Err(Error::default()),
        }
    }
}

impl
NetworkData
{
    pub fn
    network_to_memory(self) -> Result<MemoryData, Error>
    {
        Ok(MemoryData::from_network_data(self.data, self.machine_id))
    }
}

impl
MemoryData
{
    pub fn
    memory_to_network(self, id: u64) -> Result<NetworkData, Error>
    {
        match self.data
        {
            ElevatorStatusCommand::GetWorldView {world}
            => Ok(NetworkData::from_mem_data(world, id)),    
            _ => Err(Error::default())
        }
        
    }

    pub fn
    memory_to_hardware() -> Result<HardwareData, Erorr>
    {

    }
}