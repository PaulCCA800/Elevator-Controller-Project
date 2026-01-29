pub mod message
{
    const SYSTEM_IDENTIFIER: [u8; 4] = [0x0F, 0x9F, 0x8D, 0x86];

    pub enum 
    MsgType 
    {
        Broadcast   = 0,
        NACK        = 1,
        Passive     = 2,
    }

    pub struct
    UdpMsg
    {
        identifier  : [u8; 4],
        src         : Vec<u8>,
        sequence_nr : u16,
        msg_type    : MsgType,
        data        : Vec<u8>,
        checksum    : u32,
    }

    pub struct
    InternalMsg
    {
        src         : Vec<u8>,
        data        : Vec<u8>
    }

    impl
    UdpMsg
    {
        pub fn
        create(
            src: Vec<u8>, 
            packet_nr: u16, 
            msg_type: MsgType, 
            data: Vec<u8>,
            checksum: u32
        ) -> Self
        {
            Self 
            { 
                identifier  : SYSTEM_IDENTIFIER, 
                src, 
                sequence_nr : packet_nr, 
                msg_type, 
                data, 
                checksum,
            }
        }

        pub fn
        encode(&self) -> Vec<u8>
        {
            Vec::new()
        }

        pub fn
        decode(&self) -> () //UdpMsg
        {
            ()
        }
        
    }

    impl
    InternalMsg
    {
        pub fn
        create(src: Vec<u8>, data: Vec<u8>) -> Self
        {
            Self 
            { 
                src, 
                data, 
            }
        }

        pub fn
        deserialize_from_udp(&self, message: UdpMsg) -> () //InternalMsg
        {
            ()
        }
    }

}

pub struct
Msg
{
    src : String,
    sync: u8,
    data: String
}

impl 
Msg
{
    pub fn
    new(src: String, sync: u8, data: String) -> Self
    {
        Self
        {
            src,
            sync,
            data,
        }
    }

    pub fn 
    convert_msg(&self) -> Vec<u8>
    {
        let mut msg_u8 = Vec::new();
        msg_u8.push(self.sync);
        msg_u8.extend_from_slice(self.src.as_bytes());
        msg_u8.push(self.sync);
        msg_u8.extend_from_slice(self.data.as_bytes());
        msg_u8.push(self.sync);
        msg_u8
    }
}
