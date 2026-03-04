use crate::mem::WorldView;

pub struct
NetworkData
{
    source_id   : [u8; 4],
    machine_id  : u64,
    data        : WorldView
}

impl
NetworkData
{
    pub fn
    new(source_id: [u8; 4], machine_id: u64, data: WorldView) -> Self
    {
        Self
        {
            source_id,
            machine_id,
            data
        }
    }
}