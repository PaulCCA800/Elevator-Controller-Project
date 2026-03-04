use crate::mem::WorldView;

pub struct
NetworkData
{
    pub source_id   : [u8; 4],
    pub machine_id  : u64,
    pub data        : WorldView
}

impl
NetworkData
{
    pub fn
    from_mem_data()
    {

    }

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