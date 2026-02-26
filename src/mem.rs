use std::{collections::{HashMap, VecDeque}};
use crate::message::message::{MatrixCmd, Order, Direction};

#[derive(Clone)]
pub struct Elevator {
    id              : u64,
    current_floor   : u8,
    direction       : Direction,
    obstruction     : bool,
    stop            : bool,
    cab_orders      : VecDeque<Order>,
    hall_orders     : VecDeque<Order>,
    assigned_orders : VecDeque<Order>,
}

pub struct Matrix {
    matrix: HashMap <u64, Elevator>
}

impl Elevator{
    pub fn new(id: u64, current_floor: u8, direction: Direction, 
               obstruction: bool, stop: bool, cab_orders: VecDeque<Order>, 
               hall_orders: VecDeque<Order>, assigned_orders: VecDeque<Order>) -> Self{
        Self{
            id,
            current_floor,
            direction,
            obstruction,
            stop,
            cab_orders,
            hall_orders,
            assigned_orders,
        }
    }

    pub fn get_hall_orders(&self) -> &VecDeque<Order>{
        &self.hall_orders
    }

    pub fn get_cab_orders(&self) -> &VecDeque<Order>{
        &self.cab_orders
    }

    fn set_current_floor(&mut self, floor: u8) {
        self.current_floor = floor;
    }

    fn set_direction(&mut self, dir: Direction) {
        self.direction = dir;
    }

    fn set_obstruction(&mut self, obs: bool) {
        self.obstruction = obs;
    }

    fn set_stop(&mut self, stop: bool) {
        self.stop = stop;
    }

    fn set_cab_orders(&mut self, orders: VecDeque<Order>) {
        self.cab_orders = orders;
    }

    fn set_hall_orders(&mut self, orders: VecDeque<Order>) {
        self.hall_orders = orders;
    }

    pub fn set_assigned_orders(&mut self, orders: VecDeque<Order>) {
        self.assigned_orders = orders;
    }

    pub fn add_cab_order(&mut self, order: Order) {
        self.cab_orders.push_back(order);
    }

    pub fn remove_cab_order(&mut self) {
        self.cab_orders.pop_front();
    }

    pub fn add_hall_order(&mut self, order: Order) {
        self.hall_orders.push_back(order);
    }

    pub fn remove_hall_order(&mut self){
        self.hall_orders.pop_front();
    }

    pub fn add_assigned_order(&mut self, order: Order) {
        self.assigned_orders.push_back(order);
    }

    pub fn remove_assigned_order(&mut self) {
        self.assigned_orders.pop_front();
    }

}

impl Matrix {
    pub fn new(matrix: HashMap <u64, Elevator>) -> Self{
        Self {matrix}
    }

    pub fn get(&self, id: u64) -> &Elevator {
        self.matrix.get(&id).expect(&format!("get error: no elevator found at {id}."))
    }

    pub fn get_mut(&mut self, id: u64) -> &mut Elevator {
        self.matrix.get_mut(&id).expect(&format!("get_mut error: no elevator found at {id}."))
    }

    pub fn write_elev_current_floor(&mut self, id: u64, floor: u8) {
        self.get_mut(id).set_current_floor(floor);
    }

    pub fn set_elev_direction(&mut self, id: u64, direction: Direction) {
        self.get_mut(id).set_direction(direction);
    }

    pub fn set_elev_obstruction(&mut self, id: u64, obstruction: bool) {
        self.get_mut(id).set_obstruction(obstruction);
    }

    pub fn set_elev_stop(&mut self, id: u64, stop: bool) {
        self.get_mut(id).set_stop(stop);
    }

    pub fn set_elev_cab_orders(&mut self, id: u64, orders: VecDeque<Order>) {
        self.get_mut(id).set_cab_orders(orders);
    }

    pub fn set_elev_hall_orders(&mut self, id: u64, orders: VecDeque<Order>) {
        self.get_mut(id).set_hall_orders(orders);
    }

    pub fn set_elev_assigned_orders(&mut self, id: u64, orders: VecDeque<Order>) {
        self.get_mut(id).set_assigned_orders(orders);
    }

    pub fn add_elev_cab_order(&mut self, id: u64, order: Order) {
        self.get_mut(id).add_cab_order(order);
    }

    pub fn remove_elev_cab_order(&mut self, id: u64) {
        self.get_mut(id).remove_cab_order();
    }

    pub fn add_elev_hall_order(&mut self, id: u64, order: Order) {
        self.get_mut(id).add_hall_order(order);
    }

    pub fn remove_elev_hall_order(&mut self, id: u64) {
        self.get_mut(id).remove_hall_order();
    }

    pub fn add_elev_assigned_order(&mut self, id: u64, order: Order) {
        self.get_mut(id).add_assigned_order(order);
    }

    pub fn remove_elev_assigned_order(&mut self, id: u64) {
        self.get_mut(id).remove_assigned_order();
    }

    pub fn edit_matrix(&mut self, cmd: MatrixCmd) {
        match cmd { 
            MatrixCmd::SetFloor {id, floor} 
            => self.write_elev_current_floor(id, floor),

            MatrixCmd::SetDirection {id, dir} 
            => self.set_elev_direction(id, dir),

            MatrixCmd::SetObstruction {id, obs} 
            => self.set_elev_obstruction(id, obs),

            MatrixCmd::SetStop {id, stop} 
            => self.set_elev_stop(id, stop),

            MatrixCmd::SetCabOrders {id, orders} 
            => self.set_elev_cab_orders(id, orders),

            MatrixCmd::SetHallOrders {id, orders} 
            => self.set_elev_hall_orders(id, orders), 

            MatrixCmd::SetAssignedOrders {id, orders} 
             => self.set_elev_assigned_orders(id, orders),

            MatrixCmd::AddCabOrder {id, order}
            =>self.add_elev_cab_order(id, order),

            MatrixCmd::RemoveCabOrder {id}
            =>self.remove_elev_cab_order(id),

            MatrixCmd::AddHallOrder {id, order}
            =>self.add_elev_hall_order(id, order),

            MatrixCmd::RemoveHallOrder {id}
            =>self.remove_elev_hall_order(id),

            MatrixCmd::AddAssignedOrder {id, order}
            =>self.add_elev_assigned_order(id, order),

            MatrixCmd::RemoveAssignedOrder {id}
            =>self.remove_elev_assigned_order(id),

        }
    }

}





