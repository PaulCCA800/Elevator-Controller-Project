pub mod 
message
{
    const SYSTEM_IDENTIFIER: [u8; 4] = [0xF0, 0x9F, 0x8D, 0x86];

    pub enum 
    MsgType 
    {
        Broadcast   = 0,
        NACK        = 1,
        Passive     = 2,
        Corrupted   = 3,
    }

    impl 
    MsgType 
    {
        pub fn
        from_u8(val: u8) -> Self
        {
            match val
            {
                0 => MsgType::Broadcast ,
                1 => MsgType::NACK      ,
                2 => MsgType::Passive   ,
                _ => MsgType::Corrupted
            }
        }  

        pub fn
        to_u8(&self) -> u8
        {
            match self
            {
                MsgType::Broadcast  => 0,
                MsgType::NACK       => 1,
                MsgType::Passive    => 2,
                _ => 3
            }
        }  
    }

    pub struct
    UdpMsg
    {
        identifier  : [u8; 4],
        src         : u8,
        sequence_nr : u16,
        msg_type    : MsgType,
        data    : Vec<u8>,
    }

    pub struct
    InternalMsg
    {
        _src         : u8,
        _data        : Vec<u8>
    }

    impl
    UdpMsg
    {
        pub fn
        new(
            src: u8, 
            packet_nr: u16, 
            msg_type: MsgType, 
            data: Vec<u8>,
        ) -> Self
        {
            Self 
            { 
                identifier  : SYSTEM_IDENTIFIER, 
                src, 
                sequence_nr : packet_nr, 
                msg_type, 
                data, 
            }
        }

        pub fn
        encode(&mut self) -> Vec<u8>
        {
            let mut raw_data: Vec<u8>= Vec::from(self.identifier);

            raw_data.push(self.src);
            
            let [upper_byte, lower_byte] = self.sequence_nr.to_be_bytes();
            raw_data.push(upper_byte);
            raw_data.push(lower_byte);
            
            raw_data.push(self.msg_type.to_u8());
            raw_data.append(&mut self.data);

            raw_data
        }

        pub fn
        decode(buffer: Vec<u8>, byte_count: usize) -> Self //Self 
        {
            let local_identifier = [
                *buffer.get(0).unwrap(),
                *buffer.get(1).unwrap(),
                *buffer.get(2).unwrap(),
                *buffer.get(3).unwrap()
            ];

            Self
            {
                identifier: 
                    local_identifier,
                src: 
                    *buffer.get(4).unwrap(),
                sequence_nr: 
                    u16::from_le_bytes([*buffer.get(5).unwrap(), *buffer.get(6).unwrap()]),
                msg_type: 
                    MsgType::from_u8(*buffer.get(7).unwrap()),
                data:
                    Vec::from(&buffer[8..byte_count])
            }
        }
        
    }

    impl
    InternalMsg
    {
        pub fn
        new(_src: u8, _data: Vec<u8>) -> Self
        {
            Self 
            { 
                _src, 
                _data, 
            }
        }

        pub fn
        deserialize_from_udp(&self, _message: UdpMsg) -> () //InternalMsg
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
