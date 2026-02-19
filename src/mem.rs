use std::{collections::{HashMap, VecDeque}};


#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Status {
    Completed, 
    NotCompleted,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Availability {
    Taken,
    Available,
}

#[derive(Clone)]
pub struct Order {
    id: u64,
    floor: u8,
    cab: bool,
    direction: Direction,
    status: Status,
    availability: Availability,
    assigned_to: u64, 
}

#[derive(Clone)]
pub struct Elevator {
    id: u64,
    current_floor: u8,
    direction: Direction,
    obstruction: bool,
    stop: bool,
    cab_orders: VecDeque<Order>,
    hall_orders: VecDeque<Order>,
    incarnation: u8, 
}

pub struct Matrix {
    matrix: HashMap <u64, Elevator>
}

pub enum MatrixCmd {
    set_floor {id: u64, floor: u8},
    set_direction {id: u64, dir: Direction},
    set_obstruction {id: u64, obs: bool},
    set_stop {id: u64, stop: bool},
    set_cab_orders {id: u64, orders: VecDeque<Order>},
    set_hall_orders {id: u64, orders: VecDeque<Order>},
    set_assigned_orders{id: u64, orders: VecDeque<Order>},
    add_cab_order{id: u64, order: Order},
    remove_cab_order {id: u64},
    add_hall_order{id: u64, order: Order},
    remove_hall_order {id: u64},
    set_incarnation {id: u64, incarnation: u8},
    increment_incarnation {id: u64}
}

impl Order {
    pub fn new(id: u64, floor: u8, cab: bool, direction: Direction, 
               status: Status, availability: Availability, assigned_to: u64) -> Self{
        Self{
            id,
            floor,
            cab,
            direction,
            status,
            availability,
            assigned_to,
        }
    }
}

impl Elevator{
    pub fn new(id: u64, current_floor: u8, direction: Direction, 
               obstruction: bool, stop: bool, cab_orders: VecDeque<Order>, 
               hall_orders: VecDeque<Order>, incarnation: u8) -> Self{
        Self{
            id,
            current_floor,
            direction,
            obstruction,
            stop,
            cab_orders,
            hall_orders,
            incarnation,
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

    pub fn edit_matrix(&mut self, cmd: MatrixCmd) {
        match cmd { 
            MatrixCmd::set_floor {id, floor} 
            => self.write_elev_current_floor(id, floor),

            MatrixCmd::set_direction {id, dir} 
            => self.set_elev_direction(id, dir),

            MatrixCmd::set_obstruction {id, obs} 
            => self.set_elev_obstruction(id, obs),

            MatrixCmd::set_stop {id, stop} 
            => self.set_elev_stop(id, stop),

            MatrixCmd::set_cab_orders {id, orders} 
            => self.set_elev_cab_orders(id, orders),

            MatrixCmd::set_hall_orders {id, orders} 
            => self.set_elev_hall_orders(id, orders), 

            MatrixCmd::set_assigned_orders {id, orders} 
             => self.set_elev_assigned_orders(id, orders),

            MatrixCmd::add_cab_order {id, order}
            =>self.add_elev_cab_order(id, order),

            MatrixCmd::remove_cab_order {id}
            =>self.remove_elev_cab_order(id),

            MatrixCmd::add_hall_order {id, order}
            =>self.add_elev_hall_order(id, order),

            MatrixCmd::remove_hall_order {id}
            =>self.remove_elev_hall_order(id),

           

        }
    }

}





