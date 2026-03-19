use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};

use crate::memory::orders::{Order, OrderStatus};

#[derive(Serialize, Deserialize, Clone)]
pub struct HallOrderQueue{
    hall_order_queue: HashMap<u16, Order>,
}

impl HallOrderQueue{

    pub fn new() -> Self{
        Self{
            hall_order_queue: Self::initialize_hall_order_queue(),
        }
    }

    fn initialize_hall_order_queue() -> HashMap<u16, Order>{
        return HashMap::new()
    }

    pub fn get_order_queue(&self) -> &HashMap<u16, Order>{
        return &self.hall_order_queue
    }

    pub fn get_mut_hall_order_queue(&mut self) -> &mut HashMap<u16, Order>{
        return &mut self.hall_order_queue
    }

    pub fn get_hall_order(&self, order_id: u16) -> &Order{
        return self.hall_order_queue.get(&order_id)
        .expect(&format!("get error: no order found at {}.", order_id));
    }

    pub fn get_mut_hall_order(&mut self, order_id: u16) -> Option<&mut Order>{
        return self.hall_order_queue.get_mut(&order_id)
    }

    pub fn add_to_queue(&mut self, order: Order) {
        self.hall_order_queue.insert(*order.get_order_id(), order);
    }

    pub fn remove_from_queue(&mut self, order_id: u16) {
        self.hall_order_queue.remove(&order_id);
    }

    pub fn set_hall_order_status(&mut self, order_id: u16, status: OrderStatus) {
        match self.get_mut_hall_order(order_id){
            Some(order) => {order.set_order_status(status)},
            None => {println!("could not set order status for order with ID: {}", order_id)},
        }
    }

    pub fn set_hall_order_ack_barrier(&mut self, order_id: u16, barrier: Vec<u16>) {
        match self.get_mut_hall_order(order_id){
            Some(order) => {order.set_ack_barrier(barrier)},
            None => {println!("could not set ack barrier for order with ID: {}", order_id)},
        }
    }

    pub fn insert_into_hall_order_ack_barrier(&mut self, order_id: u16, elevator_id: u16) {
        match self.get_mut_hall_order(order_id){
            Some(order) => {order.insert_into_ack_barrier(elevator_id)},
            None => {println!("could not insert into ack barrier for order with ID: {}", order_id)},
        }
    }

    pub fn hall_order_status_manager(&mut self, num_elevators: u8){
        let mut orders_to_remove: Vec<u16> = Vec::new();
            for order in self.hall_order_queue.values_mut(){

                let order_id: u16 = order.get_order_id().clone();
                let unique_elevator_ids_count = order.get_ack_barrier().iter().collect::<HashSet<_>>().len();

                if order.get_order_status() == &OrderStatus::Unconfirmed && unique_elevator_ids_count == num_elevators as usize{
                    order.set_order_status(OrderStatus::Confirmed);
                    order.get_mut_ack_barrier().clear();
                }

                else if order.get_order_status() == &OrderStatus::Completed && unique_elevator_ids_count == num_elevators as usize{
                    order.set_order_status(OrderStatus::ReadyForDeletion);
                    order.get_mut_ack_barrier().clear();
                }

                else if order.get_order_status() == &OrderStatus::ReadyForDeletion && unique_elevator_ids_count == num_elevators as usize{
                    orders_to_remove.push(order_id);
                }
            }

            for order_id in orders_to_remove{
                self.remove_from_queue(order_id);
            }
    }
}