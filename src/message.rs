pub mod hardware_msg;
pub mod memory_msg;
pub mod network_msg;

use serde::{Deserialize, Serialize};

use network_msg::NetworkData;
use hardware_msg::HardwareData;
use memory_msg::MemoryData;

const LOCAL_ID : u64 = 0;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub data: MessageContent
}

#[derive(Serialize, Deserialize)]
pub enum MessageContent {
    Memory(MemoryData),
    Network(NetworkData),
    Hardware(HardwareData),
}

impl Message {
    pub fn new(content: MessageContent) -> Self {
        Self { 
            id: LOCAL_ID, 
            data: content 
        }
    }

    pub fn try_into_memory(self) -> Self {
        let mem: MemoryData = 
            match self.data {
                MessageContent::Hardware(data) => data.try_into().unwrap(),
                MessageContent::Network(data) => data.into(),
                MessageContent::Memory(data) => data,
            };

        Self { 
            id: self.id, 
            data: MessageContent::Memory(mem) 
        }
    }

    pub fn try_into_network(self) -> Result<Self, ()> {
        let mem: NetworkData =
            match self.data {
                MessageContent::Memory(data) => data.try_into()?,
                MessageContent::Network(data) => data,
                _ => return Err(()),
            };

        Ok(Self {
            id: self.id,
            data: MessageContent::Network(mem)
        })
    } 

    pub fn try_into_hardware(self) -> Result<Self, ()> {
        let mem: HardwareData =
            match self.data {
                MessageContent::Memory(data) => data.try_into().unwrap(),
                MessageContent::Hardware(data) => data,
                _ => return Err(())
            };

        Ok(Self {
            id: self.id,
            data: MessageContent::Hardware(mem)
        })
    }
}