use std::{collections::{HashMap, VecDeque}};


#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OrderStatus {
    Unconfirmed,
    Confirmed,
    Completed, 
}

#[derive(Clone)]
pub struct Order {
    order_id: u64,
    floor: u8,
    cab: bool,
    direction: Direction,
    orderStatus: OrderStatus,
    ack_barrier: Vec<u64>,
    assigned_to: u64,
}

#[derive(Clone)]
pub struct Elevator {
    elevator_id: u64,
    current_floor: u8,
    direction: Direction,
    obstruction: bool,
    stop: bool,
    cab_orders: VecDeque<Order>,
    hall_orders: VecDeque<Order>,
}

#[derive(Clone)]
pub struct WorldView {
    elevatorStatus: HashMap <u64, Elevator>,
    orderQueue: HashMap<u64, Order>,
    writeCounter: HashMap <u64, u8>,
}

pub enum ElevatorStatusCommand {
    SetFloor {elevator_id: u64, floor: u8},
    SetDirection {elevator_id: u64, dir: Direction},
    SetObstruction {elevator_id: u64, obs: bool},
    SetStop {elevator_id: u64, stop: bool},
    SetCabOrders {elevator_id: u64, orders: VecDeque<Order>},
    SetHallOrders {elevator_id: u64, orders: VecDeque<Order>},
    AddCabOrder{elevator_id: u64, order: Order},
    RemoveCabOrder {elevator_id: u64},
    AddHallOrder{elevator_id: u64, order: Order},
    RemoveHallOrder {elevator_id: u64},
}

pub enum OrderQueueCommand {
    AddToOrderQueue {order: Order},
    RemoveFromOrderQueue{order_id: u64},
    SetOrderStatus{order_id: u64, status: OrderStatus},
    SetAckBarrier{order_id: u64, barrier: Vec<u64>},
    InsertAckBarrier{order_id: u64, elevator_id: u64},
    AssignOrder{order_id: u64, elevator_id: u64},
}

impl Order {
    pub fn new(order_id: u64, floor: u8, cab: bool, direction: Direction, 
               orderStatus: OrderStatus, ack_barrier: Vec<u64>, assigned_to: u64) -> Self{
        Self{
            order_id,
            floor,
            cab,
            direction,
            orderStatus,
            ack_barrier,
            assigned_to,
        }
    }

    pub fn set_status(&mut self, status: OrderStatus) {
        self.orderStatus = status;
    }

    pub fn set_ack_barrier(&mut self, barrier: Vec<u64>) {
        self.ack_barrier = barrier;
    }

    pub fn insert_into_ack_barrier(&mut self, elevator_id: u64) {
        self.ack_barrier.push(elevator_id);
    }

    pub fn assign_to_elevator(&mut self, elevator_id: u64){
        self.assigned_to = elevator_id;
    }
}

impl Elevator{
    pub fn new(elevator_id: u64, current_floor: u8, direction: Direction, 
               obstruction: bool, stop: bool, cab_orders: VecDeque<Order>, 
               hall_orders: VecDeque<Order>) -> Self{
        Self{
            elevator_id,
            current_floor,
            direction,
            obstruction,
            stop,
            cab_orders,
            hall_orders,
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

impl WorldView {
    pub fn new(elevatorStatus: HashMap <u64, Elevator>, orderQueue: HashMap<u64, Order>, 
               writeCounter: HashMap<u64, u8>) -> Self{
        
        Self{
            elevatorStatus,
            orderQueue,
            writeCounter,
            }
    }

    //interface for elevators 

    pub fn get_elevator(&self, elevator_id: u64) -> &Elevator {
        self.elevatorStatus.get(&elevator_id).expect(&format!("get error: no elevator found at {}.", elevator_id))
    }

    pub fn get_mut_elevator(&mut self, elevator_id: u64) -> &mut Elevator {
        self.elevatorStatus.get_mut(&elevator_id).expect(&format!("get_mut error: no elevator found at {}.", elevator_id))
    }

    pub fn set_elev_current_floor(&mut self, elevator_id: u64, floor: u8) {
        self.get_mut_elevator(elevator_id).set_current_floor(floor);
    }

    pub fn set_elev_direction(&mut self, elevator_id: u64, direction: Direction) {
        self.get_mut_elevator(elevator_id).set_direction(direction);
    }

    pub fn set_elev_obstruction(&mut self, elevator_id: u64, obstruction: bool) {
        self.get_mut_elevator(elevator_id).set_obstruction(obstruction);
    }

    pub fn set_elev_stop(&mut self, elevator_id: u64, stop: bool) {
        self.get_mut_elevator(elevator_id).set_stop(stop);
    }

    pub fn set_elev_cab_orders(&mut self, elevator_id: u64, orders: VecDeque<Order>) {
        self.get_mut_elevator(elevator_id).set_cab_orders(orders);
    }

    pub fn set_elev_hall_orders(&mut self, elevator_id: u64, orders: VecDeque<Order>) {
        self.get_mut_elevator(elevator_id).set_hall_orders(orders);
    }

    pub fn add_elev_cab_order(&mut self, elevator_id: u64, order: Order) {
        self.get_mut_elevator(elevator_id).add_cab_order(order);
    }

    pub fn remove_elev_cab_order(&mut self, elevator_id: u64) {
        self.get_mut_elevator(elevator_id).remove_cab_order();
    }

    pub fn add_elev_hall_order(&mut self, elevator_id: u64, order: Order) {
        self.get_mut_elevator(elevator_id).add_hall_order(order);
    }

    pub fn remove_elev_hall_order(&mut self, elevator_id: u64) {
        self.get_mut_elevator(elevator_id).remove_hall_order();
    }

    //interface for order queue

    pub fn get_order(&self, order_id: u64) -> &Order{
        self.orderQueue.get(&order_id).expect(&format!("get error: no order found at {}.", order_id))
    }

    pub fn get_mut_order(&mut self, order_id: u64) -> &mut Order{
        self.orderQueue.get_mut(&order_id).expect(&format!("get_mut error: no order found at {}.", order_id))
    }

    pub fn add_to_queue(&mut self, order: Order) {
        self.orderQueue.insert(order.order_id, order);
    }

    pub fn remove_from_queue(&mut self, order_id: u64) {
        self.orderQueue.remove(&order_id);
    }

    pub fn set_order_status(&mut self, order_id: u64, status: OrderStatus) {
        self.get_mut_order(order_id).set_status(status);
    }

    pub fn set_order_ack_barrier(&mut self, order_id: u64, barrier: Vec<u64>) {
        self.get_mut_order(order_id).set_ack_barrier(barrier);
    }

    pub fn insert_into_order_ack_barrier(&mut self, order_id: u64, elevator_id: u64) {
        self.get_mut_order(order_id).insert_into_ack_barrier(elevator_id);
    }

    pub fn assign_order_to_elevator(&mut self, order_id: u64, elevator_id: u64) {
        self.get_mut_order(order_id).assign_to_elevator(elevator_id);
    }

    //editing functions 

    pub fn edit_elevator_status(&mut self, command: ElevatorStatusCommand) {
        match command { 
            ElevatorStatusCommand::SetFloor {elevator_id, floor} 
            => self.set_elev_current_floor(elevator_id, floor),

            ElevatorStatusCommand::SetDirection {elevator_id, dir} 
            => self.set_elev_direction(elevator_id, dir),

            ElevatorStatusCommand::SetObstruction {elevator_id, obs} 
            => self.set_elev_obstruction(elevator_id, obs),

            ElevatorStatusCommand::SetStop {elevator_id, stop} 
            => self.set_elev_stop(elevator_id, stop),

            ElevatorStatusCommand::SetCabOrders {elevator_id, orders} 
            => self.set_elev_cab_orders(elevator_id, orders),

            ElevatorStatusCommand::SetHallOrders {elevator_id, orders} 
            => self.set_elev_hall_orders(elevator_id, orders), 

            ElevatorStatusCommand::AddCabOrder {elevator_id, order}
            =>self.add_elev_cab_order(elevator_id, order),

            ElevatorStatusCommand::RemoveCabOrder {elevator_id}
            =>self.remove_elev_cab_order(elevator_id),

            ElevatorStatusCommand::AddHallOrder {elevator_id, order}
            =>self.add_elev_hall_order(elevator_id, order),

            ElevatorStatusCommand::RemoveHallOrder {elevator_id}
            =>self.remove_elev_hall_order(elevator_id),
   

        }
    }

    pub fn edit_order_queue(&mut self, command: OrderQueueCommand) {
        match command {
            OrderQueueCommand::AddToOrderQueue {order}
            => self.add_to_queue(order),

            OrderQueueCommand::RemoveFromOrderQueue {order_id}
            => self.remove_from_queue(order_id),

            OrderQueueCommand::SetOrderStatus {order_id, status}
            =>self.set_order_status(order_id, status),

            OrderQueueCommand::SetAckBarrier {order_id, barrier}
            => self.set_order_ack_barrier(order_id, barrier),

            OrderQueueCommand::InsertAckBarrier {order_id, elevator_id}
            => self.insert_into_order_ack_barrier(order_id, elevator_id),

            OrderQueueCommand::AssignOrder {order_id, elevator_id}
            => self.assign_order_to_elevator(order_id, elevator_id),
        }
    }

    pub fn increment_write_counter(&mut self, elevator_id: &u64) {
        *self.writeCounter.entry(*elevator_id).or_insert(0) += 1;
    }

}