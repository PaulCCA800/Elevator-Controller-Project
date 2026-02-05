pub mod 
message
{
    pub const SYSTEM_IDENTIFIER: [u8; 4] = [0xF0, 0x9F, 0x8D, 0x86];

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
        data        : Vec<u8>,
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
        decode(buffer: Vec<u8>, byte_count: usize) -> Option<Self> //Self 
        {
            let local_identifier = [
                *buffer.get(0).unwrap(),
                *buffer.get(1).unwrap(),
                *buffer.get(2).unwrap(),
                *buffer.get(3).unwrap()
            ];

            let data = Self
            {
                identifier: 
                    local_identifier,
                src: 
                    *buffer.get(4).unwrap(),
                sequence_nr: 
                    u16::from_be_bytes([*buffer.get(6).unwrap(), *buffer.get(5).unwrap()]),
                msg_type: 
                    MsgType::from_u8(*buffer.get(7).unwrap()),
                data:
                    Vec::from(&buffer[8..byte_count])
            };

            if data.identifier == SYSTEM_IDENTIFIER
            {
                Some(data)
            }
            else 
            {
                None    
            }
        }
        
    }


    pub enum
    Modules
    {
        Decision    = 0,
        Hardware    = 1,
        Memory      = 2,
        Network     = 3,
        Corrupted   = 99,
    }

    impl
    Modules
    {
        pub fn
        from_u8(val: u8) -> Self
        {
            match val
            {
                0 => Modules::Decision,
                1 => Modules::Hardware,
                2 => Modules::Memory,
                3 => Modules::Network,
                _ => Modules::Corrupted
            }
        }  

        pub fn
        to_u8(&self) -> u8
        {
            match self
            {
                Modules::Decision   => 0,
                Modules::Hardware   => 1,
                Modules::Memory     => 2,
                Modules::Network    => 3,
                _                   => 99,
            }
        }
    }

    pub struct
    InternalMsg
    {
        src         : Modules,
        data        : Vec<u8>
    }

    impl
    InternalMsg
    {
        pub fn
        new(src: Modules, data: Vec<u8>) -> Self
        {
            Self 
            { 
                src, 
                data, 
            }
        }

        pub fn
        from_udp(&self, message: UdpMsg) -> Self
        {
            Self
            {
                src: Modules::from_u8(message.src),
                data: message.data
            }
        }

        pub fn
        to_udp(&self, sequence_nr: u16, msg_type: MsgType) -> UdpMsg
        {
            UdpMsg{ 
                identifier  : SYSTEM_IDENTIFIER, 
                src         : self.src.to_u8(), 
                sequence_nr , 
                msg_type    , 
                data        : self.data.clone()
            }
        }
    }

}