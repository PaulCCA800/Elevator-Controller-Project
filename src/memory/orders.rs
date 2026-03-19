use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum OrderType {
    Cab,
    Hall,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub enum OrderDirection {
    Up,
    Down,
    Stop
}

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub enum OrderStatus {
    Unconfirmed,
    Confirmed,
    Completed, 
    ReadyForDeletion,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Order {
    order_id: u64,
    floor: u8,
    order_type: OrderType,
    direction: OrderDirection,
    order_status: OrderStatus,
    ack_barrier: Vec<u64>,
}

impl OrderType {
    pub fn is_cab(call: u8) -> Self {
        if call == 2 {
            OrderType::Cab
        } else {
            OrderType::Hall
        }
    }
}

impl OrderDirection {
    pub fn dir_from_call(call: u8) -> Self {
        match call
        {
            2 => OrderDirection::Down,
            1 => OrderDirection::Up,
            _ => OrderDirection::Up
        }
    }
}

impl Order {
    pub fn new(floor: u8, order_type: OrderType, direction: OrderDirection) -> Self{
        Self{
            order_id: Self::generate_order_ID(),
            floor,
            order_type,
            direction,
            order_status: OrderStatus::Unconfirmed,
            ack_barrier: Vec::new(),
        }
    }

    fn generate_order_ID() -> u64 {
        return rand::random();
    }

    pub fn get_order_id(&self) -> &u64 {
        return &self.order_id
    }

    pub fn get_floor(&self) -> &u8 {
        return &self.floor
    }

    pub fn get_order_type(&self) -> &OrderType {
        return &self.order_type
    }

    pub fn get_direction(&self) -> &OrderDirection {
        return &self.direction
    }

    pub fn get_order_status(&self) -> &OrderStatus {
        return &self.order_status
    }
    
    pub fn set_order_status(&mut self, status: OrderStatus) {
        self.order_status = status;
    }

    pub fn get_ack_barrier(&self) -> &Vec<u64>{
        return &self.ack_barrier
    }

    pub fn get_mut_ack_barrier(&mut self) -> &mut Vec<u64>{
        return &mut self.ack_barrier
    }

    pub fn set_ack_barrier(&mut self, barrier: Vec<u64>) {
        self.ack_barrier = barrier;
    }

    pub fn insert_into_ack_barrier(&mut self, elevator_id: u64) {
        self.ack_barrier.push(elevator_id);
    }
}