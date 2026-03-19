use std::thread;
use std::time::Duration;
use crossbeam_channel as cbc;
use std::collections::VecDeque;

use crate::elevator_driver::elev::{ElevatorHardware}; 
use crate::elevator_driver::elev;
use crate::memory::world_view::{MemoryCommand, ElevatorStatusCommand, OrderQueueCommand};
use crate::memory::elevator::{Elevator, Obstruction, DeadOrAlive};
use crate::memory::orders::{Order, OrderDirection, OrderStatus, OrderType};

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

                    let order = Order::new(floor, order_type, direction);

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

    loop {
        cbc::select! {
            recv(rx_elevator_state) -> msg => {
                local_elevator = Some(msg.unwrap());
            }

            recv(rx_hall_orders) -> msg => {
                assigned_hall_orders = msg.unwrap();
            }
        }

        if let Some(elevator_state) = &local_elevator {
            execute_elevator_state(
                &elevator_hw,
                elevator_state,
                &assigned_hall_orders,
                &tx_memory,
                my_elevator_id,
            );
        }
    }
}


pub fn execute_elevator_state(elevator_hw: &ElevatorHardware, elevator_state: &Elevator, assigned_hall_orders: &Vec<Order>, tx_memory: &cbc::Sender<MemoryCommand>,
my_elevator_id: u16) {
    update_lights(
        elevator_hw,
        elevator_state.get_cab_requests(),
        assigned_hall_orders,
        elevator_hw.num_floors,
    );

    if matches!(elevator_state.get_dead_or_alive(), DeadOrAlive::Dead) {
        elevator_hw.motor_direction(elev::DIRN_STOP);
        elevator_hw.door_light(false);
        return;
    }

    if elevator_state.get_obstruction() == &Obstruction::Obstructed {
        elevator_hw.motor_direction(elev::DIRN_STOP);
        elevator_hw.door_light(true);
        return;
    }

    let current_floor = elevator_state.get_floor();
    elevator_hw.floor_indicator(*current_floor);

    let mut all_orders: Vec<Order> = elevator_state
        .get_cab_requests()
        .iter()
        .cloned()
        .collect();

    all_orders.extend(assigned_hall_orders.iter().cloned());

    if all_orders.is_empty() {
        elevator_hw.motor_direction(elev::DIRN_STOP);
        elevator_hw.door_light(false);
        return;
    }

    let target_floor = choose_next_floor(*current_floor, &all_orders);

    if *current_floor == target_floor {
        elevator_hw.motor_direction(elev::DIRN_STOP);
        elevator_hw.door_light(true);

        mark_served_orders(
            elevator_state,
            assigned_hall_orders,
            *current_floor,
            tx_memory,
            my_elevator_id
        );

        thread::sleep(Duration::from_secs(3));
        elevator_hw.door_light(false);
        return;
    }

    if target_floor > *current_floor {
        elevator_hw.motor_direction(elev::DIRN_UP);
    } else {
        elevator_hw.motor_direction(elev::DIRN_DOWN);
    }
}


pub fn update_lights(elevator_hw: &ElevatorHardware, cab_orders: &VecDeque<Order>, hall_orders: &Vec<Order>, num_floors: u8) {
    for floor in 0..num_floors {
        let cab_on = cab_orders.iter().any(|o| {
            *o.get_floor() == floor
                && *o.get_order_type() == OrderType::Cab
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
    orders
        .iter()
        .min_by_key(|o| (*o.get_floor() as i16 - current_floor as i16).abs())
        .map(|o| *o.get_floor())
        .unwrap_or(current_floor)
}

pub fn mark_served_orders(elevator_state: &Elevator, assigned_hall_orders: &Vec<Order>, current_floor: u8, tx_memory: &cbc::Sender<MemoryCommand>,
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

    for order in assigned_hall_orders
        .iter()
        .filter(|o| *o.get_floor() == current_floor)
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
