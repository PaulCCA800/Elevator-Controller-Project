use std::fmt::Error;

use crate::mem::ElevatorStatusCommand;

use crate::message::{
    hardware_msg::HardwareData, 
    network_msg::NetworkData,
    memory_msg::MemoryData
};

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
    memory_to_hardware(self) -> Result<HardwareData, Error>
    {
        match self.data
        {
            _ => Err(Error::default())
        }
    }
}