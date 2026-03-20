use std::thread;
use std::time::{Duration, Instant};
use crossbeam_channel as cbc;
use std::collections::VecDeque;

use crate::elevator_driver::elev::{ElevatorHardware}; 
use crate::elevator_driver::elev;
use crate::memory::world_view::{MemoryCommand, ElevatorStatusCommand, OrderQueueCommand};
use crate::memory::elevator::{Elevator, Obstruction, DeadOrAlive, Behaviour, ElevatorDirection};
use crate::memory::orders::{Order, OrderDirection, OrderStatus, OrderType};

pub struct HardwareExecutionState {
    door_open_until: Option<Instant>,
    direction_to_clear_after_wait: Option<OrderDirection>,
    travel_target_floor: Option<u8>,
    travel_start_time: Option<Instant>,
}

impl HardwareExecutionState {
    fn new() -> Self{
        Self{
            door_open_until: None,
            direction_to_clear_after_wait: None,
            travel_target_floor: None,
            travel_start_time: None,
        }
    }
}

pub fn floor_sensor_thread(elevator_hw: ElevatorHardware, tx_memory: cbc::Sender<MemoryCommand>, my_elevator_id: u16, period: Duration){

    let mut prev = u8::MAX;
    
    loop {
        if let Some(floor) = elevator_hw.floor_sensor() {
            if floor != prev {
                tx_memory
                    .send(MemoryCommand::ElevatorStatus(
                        ElevatorStatusCommand::SetFloor {
                            elevator_id: my_elevator_id,
                            floor,
                        },
                    ))
                    .unwrap();

                prev = floor;
            }
        }
    thread::sleep(period);
    }
}


pub fn obstruction_thread( elevator_hw: ElevatorHardware, tx_memory: cbc::Sender<MemoryCommand>, my_elevator_id: u16, period: Duration,){

    let mut prev = Obstruction::Clear;

    loop {
        let obstruction = match elevator_hw.obstruction(){
            false => Obstruction::Clear,
            true => Obstruction::Obstructed,
        };

        if obstruction != prev {
            tx_memory
                .send(MemoryCommand::ElevatorStatus(
                    ElevatorStatusCommand::SetObstruction {
                        elevator_id: my_elevator_id,
                        obstruction,
                    },
                ))
                .unwrap();

            prev = obstruction;
        }

        thread::sleep(period);
    }
}


pub fn call_buttons_thread(elevator_hw: ElevatorHardware, tx_memory: cbc::Sender<MemoryCommand>, my_elevator_id: u16, period: Duration) {

    let mut prev = vec![[false; 3]; elevator_hw.num_floors as usize];

    loop {
        for floor in 0..elevator_hw.num_floors {
            for call in 0..3 {
                let pressed = elevator_hw.call_button(floor, call);

                if pressed && prev[floor as usize][call as usize] != pressed {
                    let (order_type, direction) = match call {
                        elev::HALL_UP => (OrderType::Hall, OrderDirection::Up),
                        elev::HALL_DOWN => (OrderType::Hall, OrderDirection::Down),
                        elev::CAB => (OrderType::Cab, OrderDirection::Down), //dummy for cab
                        _ => {
                            prev[floor as usize][call as usize] = pressed;
                            continue;
                        }
                    };

                    let mut order = Order::new(floor, order_type, direction);
                    order.insert_into_ack_barrier(my_elevator_id);
                    println!("NEW ORDER {:?}", &order);

                    match order_type {
                        OrderType::Cab => {
                            tx_memory
                                .send(MemoryCommand::ElevatorStatus(
                                    ElevatorStatusCommand::AddCabRequest {
                                        elevator_id: my_elevator_id,
                                        order,
                                    },
                                ))
                                .unwrap();
                        }

                        OrderType::Hall => {
                            tx_memory
                                .send(MemoryCommand::OrderQueue(
                                    OrderQueueCommand::AddToOrderQueue {order},
                                ))
                                .unwrap();
                        }
                    }
                }

                prev[floor as usize][call as usize] = pressed;
            }
        }

        thread::sleep(period);
    }
}


pub fn spawn_hardware_input_threads(elevator_hw: ElevatorHardware, tx_memory: cbc::Sender<MemoryCommand>, my_elevator_id: u16) {
    {
        let elevator_hw = elevator_hw.clone();
        let tx_memory = tx_memory.clone();
        thread::spawn(move || {
            floor_sensor_thread(
                elevator_hw,
                tx_memory,
                my_elevator_id,
                Duration::from_millis(25),
            );
        });
    }

    {
        let elevator_hw = elevator_hw.clone();
        let tx_memory = tx_memory.clone();
        thread::spawn(move || {
            obstruction_thread(
                elevator_hw,
                tx_memory,
                my_elevator_id,
                Duration::from_millis(25),
            );
        });
    }

    {
        let elevator_hw = elevator_hw.clone();
        let tx_memory = tx_memory.clone();
        thread::spawn(move || {
            call_buttons_thread(
                elevator_hw,
                tx_memory,
                my_elevator_id,
                Duration::from_millis(25),
            );
        });
    }
}

pub fn hardware_output_thread(elevator_hw: ElevatorHardware, rx_elevator_state: cbc::Receiver<Elevator>, rx_hall_orders: cbc::Receiver<Vec<Order>>, tx_memory: cbc::Sender<MemoryCommand>,
my_elevator_id: u16) {

    let mut local_elevator: Option<Elevator> = None;
    let mut assigned_hall_orders: Vec<Order> = Vec::new();
    let mut execution_state: HardwareExecutionState = HardwareExecutionState::new();
    let ticker = cbc::tick(Duration::from_millis(25));

    let mut rx_count: u64 = 0;

    loop {
        cbc::select! {
            recv(rx_elevator_state) -> msg => {
                match msg {
                    Ok(mut elevator) => {
                        while let Ok(newer) = rx_elevator_state.try_recv() {
                            elevator = newer;
                        }
                        local_elevator = Some(elevator);
                    }
                    Err(_) => {}
                }
            }

            recv(rx_hall_orders) -> msg => {
                match msg {
                    Ok(mut hall_orders) => {
                        while let Ok(newer) = rx_hall_orders.try_recv() {
                            hall_orders = newer;
                        }
                        assigned_hall_orders = hall_orders;
                    }
                    Err(_) => {}
                }
            }
            recv(ticker) -> _ => {}
        }

        if let Some(elevator_state) = &local_elevator {
            execute_elevator_state(
                &elevator_hw,
                elevator_state,
                &assigned_hall_orders,
                &tx_memory,
                my_elevator_id,
                &mut execution_state,
            );
        }
    }
}


pub fn execute_elevator_state(elevator_hw: &ElevatorHardware, elevator_state: &Elevator, assigned_hall_orders: &Vec<Order>, tx_memory: &cbc::Sender<MemoryCommand>,
my_elevator_id: u16, execution_state: &mut HardwareExecutionState) {
    update_lights(
        elevator_hw,
        elevator_state.get_cab_requests(),
        assigned_hall_orders,
        elevator_hw.num_floors,
    );

    let current_floor: u8 = *elevator_state.get_floor();
    elevator_hw.floor_indicator(current_floor);

    if matches!(elevator_state.get_dead_or_alive(), DeadOrAlive::Dead) {
        elevator_hw.motor_direction(elev::DIRN_STOP);
        elevator_hw.door_light(false);
        execution_state.door_open_until = None;
        execution_state.direction_to_clear_after_wait = None;
        execution_state.travel_target_floor = None;
        execution_state.travel_start_time = None;
        set_behaviour_if_changed(tx_memory, my_elevator_id, elevator_state, Behaviour::Idle);
        set_direction_if_changed(tx_memory, my_elevator_id, elevator_state, ElevatorDirection::Stop);
        return;
    }

    if let Some(door_open_until) = execution_state.door_open_until {
        elevator_hw.motor_direction(elev::DIRN_STOP);
        elevator_hw.door_light(true);
        execution_state.travel_target_floor = None;
        execution_state.travel_start_time = None;
        set_behaviour_if_changed(tx_memory, my_elevator_id, elevator_state, Behaviour::DoorOpen);
        set_direction_if_changed(tx_memory, my_elevator_id, elevator_state, ElevatorDirection::Stop);

        if elevator_state.get_obstruction() == &Obstruction::Obstructed {
            execution_state.door_open_until = Some(Instant::now() + Duration::from_secs(3));
            return;
        }

        if Instant::now() >= door_open_until {
            match execution_state.direction_to_clear_after_wait.take() {
                Some(direction) => {
                    mark_served_hall_orders(assigned_hall_orders,
                                            current_floor,
                                            Some(direction),
                                            tx_memory);
                    execution_state.door_open_until = Some(Instant::now() + Duration::from_secs(3));
                    return;
                }
                None => {
                    execution_state.door_open_until = None;
                    elevator_hw.door_light(false);
                }
            }
        }

        return;
    }

    let mut all_orders: Vec<Order> = elevator_state
        .get_cab_requests()
        .iter()
        .filter(|order| order.get_order_status() != &OrderStatus::Completed
                      && order.get_order_status() != &OrderStatus::ReadyForDeletion)
        .cloned()
        .collect();

    all_orders.extend(assigned_hall_orders.iter()
                                         .filter(|order| order.get_order_status() != &OrderStatus::Completed
                                                       && order.get_order_status() != &OrderStatus::ReadyForDeletion)
                                         .cloned());

    if all_orders.is_empty() {
        elevator_hw.motor_direction(elev::DIRN_STOP);
        elevator_hw.door_light(false);
        execution_state.travel_target_floor = None;
        execution_state.travel_start_time = None;
        set_behaviour_if_changed(tx_memory, my_elevator_id, elevator_state, Behaviour::Idle);
        set_direction_if_changed(tx_memory, my_elevator_id, elevator_state, ElevatorDirection::Stop);
        return;
    }

    let target_floor: u8 = choose_next_floor(current_floor, &all_orders);

    if current_floor == target_floor {
        set_dead_or_alive_if_changed(tx_memory, my_elevator_id, elevator_state, DeadOrAlive::Alive);
        execution_state.travel_target_floor = None;
        execution_state.travel_start_time = None;

        elevator_hw.motor_direction(elev::DIRN_STOP);
        elevator_hw.door_light(true);
        set_behaviour_if_changed(tx_memory, my_elevator_id, elevator_state, Behaviour::DoorOpen);
        set_direction_if_changed(tx_memory, my_elevator_id, elevator_state, ElevatorDirection::Stop);

        let (first_direction_to_clear, second_direction_to_clear) = choose_hall_directions_to_clear(elevator_state,
                                                                                                     assigned_hall_orders,
                                                                                                     &all_orders,
                                                                                                     current_floor);

        mark_served_orders(elevator_state,
                           assigned_hall_orders,
                           current_floor,
                           first_direction_to_clear,
                           tx_memory,
                           my_elevator_id);

        execution_state.door_open_until = Some(Instant::now() + Duration::from_secs(3));
        execution_state.direction_to_clear_after_wait = second_direction_to_clear;
        return;
    }

    execution_state.direction_to_clear_after_wait = None;

    let moving_direction: ElevatorDirection = if target_floor > current_floor {
        ElevatorDirection::Up
    } else {
        ElevatorDirection::Down
    };

    if execution_state.travel_target_floor != Some(target_floor) {
        execution_state.travel_target_floor = Some(target_floor);
        execution_state.travel_start_time = Some(Instant::now());
    }

    if let Some(travel_start_time) = execution_state.travel_start_time {
        if travel_start_time.elapsed() > Duration::from_secs(10) {
            set_dead_or_alive_if_changed(tx_memory, my_elevator_id, elevator_state, DeadOrAlive::Dead);
            elevator_hw.motor_direction(elev::DIRN_STOP);
            elevator_hw.door_light(false);
            set_behaviour_if_changed(tx_memory, my_elevator_id, elevator_state, Behaviour::Idle);
            set_direction_if_changed(tx_memory, my_elevator_id, elevator_state, ElevatorDirection::Stop);
            return;
        }
    }

    set_dead_or_alive_if_changed(tx_memory, my_elevator_id, elevator_state, DeadOrAlive::Alive);
    set_behaviour_if_changed(tx_memory, my_elevator_id, elevator_state, Behaviour::Moving);
    set_direction_if_changed(tx_memory, my_elevator_id, elevator_state, moving_direction);
    elevator_hw.door_light(false);

    match moving_direction {
        ElevatorDirection::Up => elevator_hw.motor_direction(elev::DIRN_UP),
        ElevatorDirection::Down => elevator_hw.motor_direction(elev::DIRN_DOWN),
        ElevatorDirection::Stop => elevator_hw.motor_direction(elev::DIRN_STOP),
    }
}

fn set_behaviour_if_changed(tx_memory: &cbc::Sender<MemoryCommand>, my_elevator_id: u16, elevator_state: &Elevator, behaviour: Behaviour) {
    if elevator_state.get_behaviour() != &behaviour {
        tx_memory.send(MemoryCommand::ElevatorStatus(
            ElevatorStatusCommand::SetBehaviour {
                elevator_id: my_elevator_id,
                behavior: behaviour,
            },
        )).unwrap();
    }
}

fn set_direction_if_changed(tx_memory: &cbc::Sender<MemoryCommand>, my_elevator_id: u16, elevator_state: &Elevator, direction: ElevatorDirection) {
    if elevator_state.get_direction() != &direction {
        tx_memory.send(MemoryCommand::ElevatorStatus(
            ElevatorStatusCommand::SetDirection {
                elevator_id: my_elevator_id,
                dir: direction,
            },
        )).unwrap();
    }
}

fn set_dead_or_alive_if_changed(tx_memory: &cbc::Sender<MemoryCommand>, my_elevator_id: u16, elevator_state: &Elevator, dead_or_alive: DeadOrAlive) {
    if elevator_state.get_dead_or_alive() != &dead_or_alive {
        tx_memory.send(MemoryCommand::ElevatorStatus(
            ElevatorStatusCommand::SetDeadOrAlive {
                elevator_id: my_elevator_id,
                dead_or_alive,
            },
        )).unwrap();
    }
}

pub fn update_lights(elevator_hw: &ElevatorHardware, cab_orders: &VecDeque<Order>, hall_orders: &Vec<Order>, num_floors: u8) {
    for floor in 0..num_floors {
        let cab_on = cab_orders.iter().any(|o| {
            *o.get_floor() == floor
                && *o.get_order_type() == OrderType::Cab
                && *o.get_order_status() != OrderStatus::Completed
                && *o.get_order_status() != OrderStatus::ReadyForDeletion
        });

        let hall_up_on = hall_orders.iter().any(|o| {
            *o.get_floor() == floor
                && *o.get_order_type() == OrderType::Hall
                && *o.get_direction() == OrderDirection::Up
                && *o.get_order_status() == OrderStatus::Confirmed
        });

        let hall_down_on = hall_orders.iter().any(|o| {
            *o.get_floor() == floor
                && *o.get_order_type() == OrderType::Hall
                && *o.get_direction() == OrderDirection::Down
                && *o.get_order_status() == OrderStatus::Confirmed
        });

        elevator_hw.call_button_light(floor, elev::CAB, cab_on);
        elevator_hw.call_button_light(floor, elev::HALL_UP, hall_up_on);
        elevator_hw.call_button_light(floor, elev::HALL_DOWN, hall_down_on);
    }
}


pub fn choose_next_floor(current_floor: u8, orders: &Vec<Order>) -> u8 {
    return orders
        .iter()
        .min_by_key(|o| (*o.get_floor() as i16 - current_floor as i16).abs())
        .map(|o| *o.get_floor())
        .unwrap_or(current_floor)
}

fn choose_hall_directions_to_clear(elevator_state: &Elevator, assigned_hall_orders: &Vec<Order>, all_orders: &Vec<Order>, current_floor: u8)
-> (Option<OrderDirection>, Option<OrderDirection>) {
    let hall_up_at_floor = assigned_hall_orders.iter().any(|order| {
        *order.get_floor() == current_floor && *order.get_direction() == OrderDirection::Up
    });

    let hall_down_at_floor = assigned_hall_orders.iter().any(|order| {
        *order.get_floor() == current_floor && *order.get_direction() == OrderDirection::Down
    });

    let orders_above = all_orders.iter().any(|order| *order.get_floor() > current_floor);
    let orders_below = all_orders.iter().any(|order| *order.get_floor() < current_floor);

    match (hall_up_at_floor, hall_down_at_floor) {
        (true, true) => {
            if orders_above {
                return (Some(OrderDirection::Up), None);
            }
            if orders_below {
                return (Some(OrderDirection::Down), None);
            }

            match elevator_state.get_direction() {
                ElevatorDirection::Down => return (Some(OrderDirection::Down), Some(OrderDirection::Up)),
                ElevatorDirection::Up => return (Some(OrderDirection::Up), Some(OrderDirection::Down)),
                ElevatorDirection::Stop => return (Some(OrderDirection::Up), Some(OrderDirection::Down)),
            }
        }
        (true, false) => return (Some(OrderDirection::Up), None),
        (false, true) => return (Some(OrderDirection::Down), None),
        (false, false) => return (None, None),
    }
}

pub fn mark_served_orders(elevator_state: &Elevator, assigned_hall_orders: &Vec<Order>, current_floor: u8, hall_direction: Option<OrderDirection>, tx_memory: &cbc::Sender<MemoryCommand>,
    my_elevator_id: u16) {
    for order in elevator_state
        .get_cab_requests()
        .iter()
        .filter(|o| *o.get_floor() == current_floor)
    {
        tx_memory
            .send(MemoryCommand::ElevatorStatus(
                ElevatorStatusCommand::SetCabOrderStatus {
                    elevator_id: my_elevator_id,
                    order_id: *order.get_order_id(),
                    status: OrderStatus::Completed,
                },
            ))
            .unwrap();
    }

    mark_served_hall_orders(assigned_hall_orders, current_floor, hall_direction, tx_memory);
}

fn mark_served_hall_orders(assigned_hall_orders: &Vec<Order>, current_floor: u8, hall_direction: Option<OrderDirection>, tx_memory: &cbc::Sender<MemoryCommand>) {
    for order in assigned_hall_orders
        .iter()
        .filter(|o| {
            *o.get_floor() == current_floor
            && match hall_direction {
                Some(direction) => o.get_direction() == &direction,
                None => false,
            }
        })
    {
        tx_memory
            .send(MemoryCommand::OrderQueue(
                OrderQueueCommand::SetOrderStatus {
                    order_id: *order.get_order_id(),
                    status: OrderStatus::Completed,
                },
            ))
            .unwrap();
    }
}
