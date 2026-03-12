use crate::mem::WorldView;

const SYSTEM_IDENTIFIER: [u8; 4] = [0xF0, 0x9F, 0x8D, 0x86];

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct NetworkData {
    pub source_id   : [u8; 4],
    pub machine_id  : u64,
    pub data        : WorldView
}

impl NetworkData {
    pub fn new(data: WorldView, machine_id: u64) -> Self {
        Self {
            source_id: SYSTEM_IDENTIFIER, 
            machine_id, 
            data 
        }
    }

    pub fn from_mem_data(data: WorldView, id: u64) -> NetworkData {
        NetworkData::new(data, id)
    }
}