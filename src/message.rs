pub mod hardware_msg;
pub mod memory_msg;
pub mod network_msg;
pub mod translate;

use serde::{Deserialize, Serialize};

use network_msg::NetworkData;
use hardware_msg::HardwareData;
use memory_msg::MemoryData;

#[derive(Serialize, Deserialize)]
pub struct
Message
{
    pub id: u64,
    pub data: MessageContent
}

#[derive(Serialize, Deserialize)]
pub enum
MessageContent
{
    Memory  (MemoryData),
    Network (NetworkData),
    Hardware(HardwareData),
}