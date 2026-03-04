use std::fmt::Error;

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
                => Err(Error::default()),
            Self::DataFloorSensor { floor } 
                => Err(Error::default()),
            Self::DataObstruction { status }  
                => Err(Error::default()),
            Self::DataStopButton { status } 
                => Err(Error::default()),
            _ => Err(Error::default()),
        }
    }
}

impl
NetworkData
{

}

impl
MemoryData
{

}