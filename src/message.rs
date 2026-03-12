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
    pub fn new_local(content: MessageContent) -> Self {
        Self { 
            id: LOCAL_ID, 
            data: content 
        }
    }    
}