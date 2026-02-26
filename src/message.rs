use std::{fmt::Error, vec};

use driver_rust::elevio::poll::CallButton;

use crate::mem::{Direction, ElevatorStatusCommand, OrderStatus, Order};

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
    data        : Vec<u8>
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
                match hardware_data
                {
                    HardwareData::GetCallButton {call_button_data}
                    => {
                        self.data = 
                        MessageContent::Memory(
                        MemoryData{ 
                        data: ElevatorStatusCommand::AddOrder
                        {   
                            elevator_id: self.id, 
                            order: Order::new(
                                self.id, 
                                call_button_data.floor, 
                                is_cab(call_button_data.call),
                                get_dir(call_button_data.call),
                                OrderStatus::Unconfirmed, 
                                vec![], 
                                self.id),
                            }});
                        Some(self)
                    },
                    HardwareData::GetFloorSensor{floor}
                    => 
                    {
                        self.data = MessageContent::Memory(
                            MemoryData{ 
                                data: ElevatorStatusCommand::SetFloor{ 
                                    elevator_id: self.id, 
                                    floor
                                }});
                        Some(self)
                    },
                    HardwareData::GetObstruction { status }
                    => {
                        self.data = MessageContent::Memory(
                            MemoryData { 
                                data: ElevatorStatusCommand::SetObstruction 
                                    { 
                                        elevator_id: self.id, 
                                        obs: status
                                    }});
                        Some(self)
                    },
                    HardwareData::GetStopButton { status } 
                    => {
                        self.data = MessageContent::Memory(
                            MemoryData { 
                                data: ElevatorStatusCommand::SetStop 
                                    { 
                                        elevator_id: self.id, 
                                        stop: status
                                    }});
                        Some(self)
                    }, 
                    _ => None
                }
            },
            MessageContent::Network(network_data)
            => 
            {
                
                None    
            }
            //todo: add actual conversion
            ,
            _ 
            => Some(self),
        }
    }

    /*
    pub fn
    to_network(self) -> Result<MessageContent, Error>
    {
        //Ok(self)
    }

    pub fn
    to_hardware(self) -> Result<MessageContent, Error>
    {
        //Ok(self)
    }
     */
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


    #[derive(Copy, Clone, Eq, PartialEq)]
    pub enum Direction {
        Up,
        Down,
    }

    #[derive(Clone)]
    pub struct Order {
        id          : u64,
        floor       : u8,
        cab         : bool,
        direction   : Direction,
    }

    impl Order {
        pub fn new(id: u64, floor: u8, cab: bool, direction: Direction) -> Self{
            Self{
                id,
                floor,
                cab,
                direction,
            }
        }
    }

    pub enum MatrixCmd {
        SetFloor            {id: u64, floor: u8},
        SetDirection        {id: u64, dir: Direction},
        SetObstruction      {id: u64, obs: bool},
        SetStop             {id: u64, stop: bool},
        SetCabOrders        {id: u64, orders: VecDeque<Order>},
        SetHallOrders       {id: u64, orders: VecDeque<Order>},
        SetAssignedOrders   {id: u64, orders: VecDeque<Order>},
        AddCabOrder         {id: u64, order: Order},
        RemoveCabOrder      {id: u64},
        AddHallOrder        {id: u64, order: Order},
        RemoveHallOrder     {id: u64},
        AddAssignedOrder    {id: u64, order: Order},
        RemoveAssignedOrder {id: u64},
    }

    /*
    impl 
    ElevatorUpdateMsg {
        pub fn
        to_matrix_command(&self) -> MatrixCmd
        {
            match self
            {
                ElevatorUpdateMsg::FloorSensor(floor) 
                    => MatrixCmd::SetFloor { id: 0, floor: *floor},
                ElevatorUpdateMsg::CallButton(call_button)    
                    => MatrixCmd::AddAssignedOrder { id: 0, order: Order{id: 0, floor: call_button.floor, direction: call_button.call} },
                ElevatorUpdateMsg::Obstruction(obs)
                    => MatrixCmd::SetObstruction { id: 0, obs: *obs },
                _ 
                => MatrixCmd::SetStop { id: _, stop: _ },
            }
        }    
    }

    impl 
    ElevatorCommand
    {
        
        pub fn
        to_matrix_command(&self) -> MatrixCmd
        {
            match self
            {
                
            }
        }
        
        pub fn
        from_matrix_command(cmd: MatrixCmd) -> ElevatorCommand
        {

        }
    }
    */
}