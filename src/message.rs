use std::vec;

use driver_rust::elevio::poll::CallButton;
use serde::{Deserialize, Serialize};

use crate::mem::{Direction, ElevatorStatusCommand, Order, OrderStatus, WorldView};

pub struct
Message
{
    id: u64,
    data: MessageContent
}

pub enum
MessageContent
{
    Memory  (MemoryData),
    Network (NetworkData),
    Hardware(HardwareData),
}

pub struct
MemoryData
{
    data: ElevatorStatusCommand,
}

pub struct
NetworkData
{
    source_id   : [u8; 4],
    machine_id  : u64,
    data        : WorldView
}

pub enum
HardwareData
{
    GetCallButton       {call_button_data: CallButton},
    GetFloorSensor      {floor: u8},
    GetStopButton       {status: bool},
    GetObstruction      {status: bool},
    SetMotorDirection   {dir: u8},
    SetCallButtonLight  {floor: u8, call: u8, status: bool},
    SetDoorLight        {status: bool},
    SetStopLight        {status: bool},
    SetFloorIndicator   {floor: u8},
}

impl 
Message
{
    pub fn
    to_memory(mut self) -> Option<Message>
    {
        match self.data
        {
            MessageContent::Hardware(hardware_data)
            => 
            {
                if let Some(translated_message_content) = Message::hardware_to_memory(hardware_data, self.id){
                    self.data = translated_message_content;
                    Some(self)
                } else {
                    None
                }
            },
            MessageContent::Network(network_data)
            => 
            {
                None    
            },
            _ 
            => Some(self),
        }
    }

    pub fn
    to_hardware(mut self) -> Option<Message>
    {
        match self.data
        {
            MessageContent::Memory(elevator_command)
            => {
                match elevator_command.data
                {
                    ElevatorStatusCommand::SetFloor{elevator_id, floor } 
                    => {
                        None
                    },
                    ElevatorStatusCommand::SetDirection{elevator_id, dir }
                    => {
                        self.id     = elevator_id;
                        self.data   = MessageContent::Hardware(
                            HardwareData::SetMotorDirection {dir: dir.to_u8()}
                        );
                        Some(self)
                    },
                    ElevatorStatusCommand::SetStop{elevator_id, stop }
                    => {
                        self.id = elevator_id;
                        self.data = MessageContent::Hardware(
                            HardwareData::SetStopLight { status: stop }
                        );
                        Some(self)
                    },
                    _ => None,
                }
            },
            _ => None
        }
    }

    pub fn
    to_network(mut self) -> Option<()>
    {
        match self.data
        {
            MessageContent::Memory(memory_data) 
            => Some(()),
            _ => None
        }
    }

    fn
    hardware_to_memory(hardware_data: HardwareData, id: u64) -> Option<MessageContent>
    {
        match hardware_data
        {
        HardwareData::GetCallButton {call_button_data}
        => 
        {
            let output = MessageContent::Memory(
            MemoryData{ 
            data: ElevatorStatusCommand::AddOrder
            {   
                elevator_id: id, 
                order: Order::new(
                    id, 
                    call_button_data.floor, 
                    is_cab(call_button_data.call),
                    get_dir(call_button_data.call),
                    OrderStatus::Unconfirmed, 
                    vec![], 
                    id),
                }});
            Some(output)
        },
        HardwareData::GetFloorSensor{floor}
        => 
        {
            let output = MessageContent::Memory(
            MemoryData{ 
            data: ElevatorStatusCommand::SetFloor{ 
                elevator_id: id, 
                floor
            }});
            Some(output)
        },
        HardwareData::GetObstruction { status }
        => 
        {
            let output = MessageContent::Memory(
            MemoryData { 
            data: ElevatorStatusCommand::SetObstruction{ 
                elevator_id: id, 
                obs: status
            }});
            Some(output)
        },
        HardwareData::GetStopButton { status } 
        => 
        {
            let output = MessageContent::Memory(
            MemoryData { 
            data: ElevatorStatusCommand::SetStop{ 
                elevator_id: id, 
                stop: status
            }});
            Some(output)
        },
        _ => None
        }
    }
}

fn
is_cab(call: u8) -> bool
{
    return call == 2;
}

fn
get_dir(call: u8) -> Direction
{
    if call == 2
    {
        return Direction::Inherit;
    }
    else if call == 1
    {
        return Direction::Down
    }
    return Direction::Up;
}

pub mod message
{
    use std::collections::VecDeque;

    use driver_rust::elevio::poll::CallButton;

    pub const SYSTEM_IDENTIFIER: [u8; 4] = [0xF0, 0x9F, 0x8D, 0x86];

    #[derive(Debug)]
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

    #[derive(Debug)]
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

    #[derive(Debug)]
    pub enum
    ElevatorUpdateMsg
    {
        CallButton  (CallButton),
        FloorSensor (u8),
        StopButton  (bool),
        Obstruction (bool)
    }

    pub enum
    ElevatorCommand
    {    
        SetMotorDirection   {dir: u8},
        SetCallButtonLight  {floor: u8, call: u8, status: bool},
        SetDoorLight        {status: bool},
        SetStopLight        {status: bool},
        SetFloorIndicator   {floor: u8},
    }
}