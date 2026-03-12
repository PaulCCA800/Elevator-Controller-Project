use std::convert::TryFrom;

use crate::{memory::{ElevatorStatusCommand, WorldView}, message::memory_msg::MemoryData};

const SYSTEM_IDENTIFIER: [u8; 4] = [0xF0, 0x9F, 0x8D, 0x86];

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct NetworkData {
    pub source_id   : [u8; 4],
    pub machine_id  : u64,
    pub data        : WorldView
}

impl TryFrom<MemoryData> for NetworkData {
    type Error = ();

    fn try_from(data: MemoryData) -> Result<Self, Self::Error> {
        match data.data {
            //ElevatorStatusCommand::GetWorldView{elevator_id, world} => Ok(
            //    NetworkData::new(world, elevator_id)
            //),
            _ => Err(())
        }
    }
}

impl NetworkData {
    pub fn new(data: WorldView, machine_id: u64) -> Self {
        Self {
            source_id: SYSTEM_IDENTIFIER, 
            machine_id, 
            data 
        }
    }
}