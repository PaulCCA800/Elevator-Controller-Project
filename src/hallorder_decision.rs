use crossbeam_channel as cbc;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, VecDeque};
use std::process::{Command};

use crate::memory::elevator::{Behaviour, DeadOrAlive, Elevator, ElevatorDirection, Obstruction};
use crate::memory::order::{Order, OrderDirection, OrderType, OrderStatus};
use crate::memory::world_view::WorldView;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElevatorState {
    behaviour: Behaviour,
    floor: u8,
    direction: ElevatorDirection,
    cab_requests: Vec<bool>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    hall_requests: [[bool; 2]; 4],
    states: HashMap<String, ElevatorState>,
}


pub type Output = HashMap<String, Vec<[bool; 3]>>;


impl Input {
    pub fn new(hall_requests: [[bool; 2]; 4], states: HashMap<String, ElevatorState>) -> Self {
        Self {
            hall_requests,
            states,
        }
    }
}

impl ElevatorState {
    pub fn new(behaviour: Behaviour, floor: u8, direction: ElevatorDirection, cab_requests: Vec<bool>) -> Self {
        Self {
            behaviour,
            floor,
            direction,
            cab_requests,
        }
    }
}

fn assigner(input: &Input, exe_path: &String) -> anyhow::Result<Output> {
    let output = Command::new(exe_path)
        .arg("-i")
        .arg(serde_json::to_string(input)?)
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "assigner failed: {}\nstderr: {}\nstdout: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout),
        ));
    }

    if output.stdout.is_empty() {
        return Err(anyhow::anyhow!(
            "assigner returned empty stdout\nstderr: {}",
            String::from_utf8_lossy(&output.stderr),
        ));
    }

    let assigner_output: Output = serde_json::from_slice(&output.stdout)?;
    Ok(assigner_output)
}


fn assigner_output_to_assigned_orders(
    orderMap: HashMap<String, 
    Vec<[bool; 3]>>, 
    hallOrderMap: &HashMap<u16, Order>
) -> HashMap<u16, VecDeque<Order>> {

    let mut assignedOrdersByID: HashMap<u16, VecDeque<Order>> = HashMap::new(); 
    for (elevator, orders) in orderMap {
        let id: u16 = match elevator.parse() {
            Ok(elevatorID) => elevatorID,
            Err(e) => {print!("Parse failed {}. Returning empty hashmap", e);
                                      return HashMap::new();}
        };
        let mut assignedOrders: VecDeque<Order> = VecDeque::new();  
        for (floor, entry) in orders.into_iter().enumerate() {
            if entry[0] == true {
                match hallOrderMap.values().find(|order| *order.get_floor() as usize == floor
                                                      && *order.get_order_type() == OrderType::Hall
                                                      && *order.get_direction() == OrderDirection::Up) {
                    Some(order) => {assignedOrders.push_back(order.clone())},
                    None => {},
                }
            }
            if entry[1] == true {
                match hallOrderMap.values().find(|order| *order.get_floor() as usize == floor
                                                      && *order.get_order_type() == OrderType::Hall
                                                      && *order.get_direction() == OrderDirection::Down) {
                    Some(order) => {assignedOrders.push_back(order.clone())},
                    None => {},
                }
            }
        };
        assignedOrdersByID.insert(id, assignedOrders);
    }

    return assignedOrdersByID;
}  


fn hall_order_format_converter(order_queue: &HashMap<u16, Order>) -> [[bool; 2]; 4] {
    let mut queue = [[false; 2]; 4];
    for order in order_queue.values(){
        let dir_idx = match order.get_direction() {
            OrderDirection::Up => 0,
            OrderDirection::Down => 1,
        };
        queue[*order.get_floor() as usize][dir_idx] = true;
    }
    return queue
}


fn cab_order_format_converter(order_queue: &VecDeque<Order>) -> Vec<bool> {
    let mut queue: Vec<bool> = vec![false; 4];
    for order in order_queue {
        queue[*order.get_floor() as usize] = true;
    }
    return queue
}


pub fn assign_hall_orders(last_world_view: WorldView) -> anyhow::Result<HashMap<u16, VecDeque<Order>>> {
    let mut states: HashMap<String, ElevatorState> = HashMap::new();

    let filtered_elevators: HashMap<u16, Elevator> = last_world_view
        .get_elevator_statuses()
        .iter()
        .filter(|(_, elevator)| elevator.get_dead_or_alive() == &DeadOrAlive::Alive 
        && elevator.get_obstruction() == &Obstruction::Clear)
        .map(|(id, elevator)| (*id, elevator.clone()))
        .collect();

    for (id, elevator) in filtered_elevators {
        let id_string: String = id.to_string();
        let cab_requests: Vec<bool> = cab_order_format_converter(elevator.get_cab_orders());
        let state: ElevatorState = ElevatorState::new(
            elevator.get_behaviour().clone(),
            elevator.get_floor().clone(),
            elevator.get_direction().clone(),
            cab_requests,
        );
        states.insert(id_string, state);
    }

    let filtered_hall_orders: HashMap<u16, Order> = last_world_view
        .get_hall_order_queue()
        .get_order_queue()
        .iter()
        .filter(|(_, order)| order.get_order_status() == &OrderStatus::Confirmed)
        .map(|(id, order)| (*id, order.clone()))
        .collect();

    let queue: [[bool; 2]; 4] = hall_order_format_converter(&filtered_hall_orders);
    let input: Input = Input::new(queue, states);
    let path_to_assigner: String = "./bin/hall_request_assigner".to_string();

    let assigned_hall_orders = assigner(&input, &path_to_assigner)?;

    Ok(assigner_output_to_assigned_orders(
        assigned_hall_orders,
        &filtered_hall_orders,
    ))
}


pub fn decision_thread(rx_decision: cbc::Receiver<WorldView>, tx_hall_orders: cbc::Sender<Vec<Order>>, my_elevator_id: u16) {

    let mut assigned_hall_orders: HashMap<u16, VecDeque<Order>> = HashMap::new();

    let mut last_sent_hall_orders: Option<Vec<Order>> = None;

    loop {
        match rx_decision.recv() {
            Ok(mut world_view) => {
                while let Ok(newer) = rx_decision.try_recv() {
                    world_view = newer;
                }

                match assign_hall_orders(world_view) {
                    Ok(orders) => {
                        assigned_hall_orders = orders;
                    }
                    Err(e) => {
                        continue;
                    }
                }

                let my_filtered_hall_orders: Vec<Order> = assigned_hall_orders
                    .get(&my_elevator_id)
                    .cloned()
                    .unwrap_or_else(VecDeque::new)
                    .into_iter()
                    .collect();

                if last_sent_hall_orders.as_ref() != Some(&my_filtered_hall_orders) {
                    tx_hall_orders.send(my_filtered_hall_orders.clone()).unwrap();
                    last_sent_hall_orders = Some(my_filtered_hall_orders);
                }
            }

            Err(_) => {
                println!("failed to receive on rx_decision for elevator {}", my_elevator_id);
            }
        }
    }
}
