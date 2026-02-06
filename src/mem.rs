use std::{collections::HashMap}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
}

#[derive(Clone)]
pub struct Order {
    id: u64,
    floor: u8,
    cab: bool,
    direction: Direction,
}

#[derive(Clone)]
pub struct Elevator {
    id: u64,
    current_floor: u8,
    direction: Direction,
    obstruction: bool,
    stop: bool,
    cab_orders: Vec<Order>,
    hall_orders: Vec<Order>,
}

pub struct Matrix {
    matrix: HashMap <u64, Elevator>
}

pub enum MatrixCmd {
    set_floor {id: u64, floor: u8},
    set_direction {id: u64, dir: Direction},
    set_obstruction {id: u64, obs: bool},
    set_stop {id: u64, stop: bool},
    set_cab_orders {id: u64, orders: Vec<Order>},
    set_hall_orders {id: u64, orders: Vec<Order>}
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

impl Elevator{
    pub fn new(id: u64, current_floor: u8, direction: Direction, 
               obstruction: bool, stop: bool, cab_orders: Vec<Order>, 
               hall_orders: Vec<Order>) -> Self{
        Self{
            id,
            current_floor,
            direction,
            obstruction,
            stop,
            cab_orders,
            hall_orders,
        }
    }

    pub fn get_hall_orders(&self) -> &Vec<Order>{
        &self.hall_orders
    }

    pub fn get_cab_orders(&self) -> &Vec<Order>{
        &self.cab_orders
    }

    fn set_current_floor(&mut self, floor: u8) {
        self.current_floor = floor;
    }

    fn set_direction(&mut self, dir: &Direction) {
        self.direction = dir;
    }

    fn set_obstruction(&mut self, obs: &bool) {
        self.obstruction = obs;
    }

    fn set_stop(&mut self, stop: &bool) {
        self.stop = stop;
    }

    fn set_cab_orders(&mut self, orders: &[Order]) {
        self.cab_orders = orders.to_vec();
    }

    fn set_hall_orders(&mut self, orders: &[Order]) {
        self.hall_orders = orders.to_vec();
    }
}

impl Matrix {
    pub fn new(matrix: HashMap <u64, Elevator>) -> Self{
        Self {matrix}
    }

    pub fn get(&self, id: u64) -> &Elevator {
        self.matrix.get(id).expect(&format!("get error: no elevator found at {id}.", id))
    }

    pub fn get_mut(&mut self, id: u64) -> &mut Elevator {
        self.matrix.get_mut(id).expect(&format!("get_mut error: no elevator found at {id}.", id))
    }

    pub fn write_elev_current_floor(&mut self, id: u64, floor: u8) {
        self.get_mut(id).set_current_floor(floor);
    }

    pub fn set_elev_direction(&mut self, id: u64, direction: &Direction) {
        self.get_mut(id).set_direction(direction);
    }

    pub fn set_elev_obstruction(&mut self, id: u64, obstruction: &bool) {
        self.get_mut(id).set_obstruction(obstruction);
    }

    pub fn set_elev_stop(&mut self, id: u64, stop: &bool) {
        self.get_mut(id).set_stop(stop);
    }

    pub fn set_elev_cab_orders(&mut self, id: u64, orders: &[Order]) {
        self.get_mut(id).set_cab_orders(orders);
    }

    pub fn set_elev_hall_orders(&mut self, id: u64, orders: &[Order]) {
        self.get_mut(id).set_hall_orders(orders);
    }

    pub fn edit_matrix(&mut self, cmd: MatrixCmd) {
        match cmd { 
            MatrixCmd::set_floor {id, floor} 
            => self.write_elev_current_floor(id, floor),

            MatrixCmd::set_direction {id, dir} 
            => self.set_elev_direction(id, &dir),

            MatrixCmd::set_obstruction {id, obs} 
            => self.set_elev_obstruction(id, &obs),

            MatrixCmd::set_stop {id, stop} 
            => self.set_elev_stop(id, &stop),

            MatrixCmd::set_cab_orders {id, orders} 
            => self.set_elev_cab_orders(id, &orders.as_slice()),

            MatrixCmd::set_hall_orders {id, orders} 
            => self.set_elev_hall_orders(id, &orders.as_slice()),
        }
    }

}





