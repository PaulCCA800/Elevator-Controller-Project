use std::{collections::HashMap, fmt::format, option}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Stop,
}
#[derive(Clone)]
struct Order {
    id: u8,
    floor: u8,
    cab: bool,
    direction: Direction,
}

struct Elevator {
    id: u8,
    current_floor: u8,
    direction: Direction,
    obstruction: bool,
    stop: bool,
    cab_orders: Vec<Order>,
    hall_orders: Vec<Order>,
}

struct Matrix {
    matrix: HashMap <u8, Elevator> 
}

impl Order {
    pub fn new(id: u8, floor: u8, cab: bool, direction: Direction) -> Self{
        Self{
            id,
            floor,
            cab,
            direction,
        }
    }
}

impl Elevator{
    pub fn new(id: u8, current_floor: u8, direction: Direction, 
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
    pub fn new(matrix: HashMap <u8, Elevator>) -> Self{
        Self {matrix}
    }

    fn get(&self, id: u8) -> &Elevator {
        self.matrix.get(id).expect(&format!("get error: no elevator found at {id}.", id))
    }

    fn get_mut(&mut self, id: u8) -> &mut Elevator {
        self.matrix.get_mut(id).expect(&format!("get_mut error: no elevator found at {id}.", id))
    }

    fn write_elev_current_floor(&mut self, id: u8, floor: u8) {
        self.get_mut(id).set_current_floor(floor);
    }

    fn set_elev_direction(&mut self, id: u8, direction: &Direction) {
        self.get_mut(id).set_direction(direction);
    }

    fn set_elev_obstruction(&mut self, id: u8, obstruction: &bool) {
        self.get_mut(id).set_obstruction(obstruction);
    }

    fn set_elev_stop(&mut self, id: u8, stop: &bool) {
        self.get_mut(id).set_stop(stop);
    }

    fn set_elev_cab_orders(&mut self, id: u8, orders: &[Order]) {
        self.get_mut(id).set_cab_orders(orders);
    }

    fn set_elev_hall_orders(&mut self, id: u8, orders: &[Order]) {
        self.get_mut(id).set_hall_orders(orders);
    }

}

fn send_to_channel() {

}

fn recv_from_channel() {

}



