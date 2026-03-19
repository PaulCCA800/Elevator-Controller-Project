use std::collections::{HashMap, VecDeque};
use std::io::Write;
use std::process::{Command, Stdio};
use serde::{Serialize, Deserialize};
use crossbeam_channel as cbc;

use crate::memory::elevator::{Elevator, ElevatorDirection, DeadOrAlive, Behaviour};
use crate::memory::orders::{Order, OrderDirection, OrderType, OrderStatus};
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

pub type Output = HashMap<String, Vec<[bool; 2]>>;

impl Input {
    pub fn new(hall_requests: [[bool; 2]; 4], states: HashMap<String, ElevatorState>) -> Self {
        Self{
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
    let mut hall_assigner = Command::new(exe_path)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()?;

    let json_input = serde_json::to_vec(input)?;
    hall_assigner.stdin.as_mut().unwrap().write_all(&json_input)?;
    drop(hall_assigner.stdin.take());

    let output = hall_assigner.wait_with_output()?;
    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "assigner failed: {}\nstderr: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let assignments: Output = serde_json::from_slice(&output.stdout)?;
    return Ok(assignments)
}

fn assigner_output_to_assigned_orders(orderMap: HashMap<String, Vec<[bool; 2]>>, hallOrderMap: &HashMap<u16, Order>) -> HashMap<u16, VecDeque<Order>>{

    let mut assignedOrdersByID: HashMap<u16, VecDeque<Order>> = HashMap::new(); 
    for (elevator, orders) in orderMap{
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


fn hall_order_format_converter(order_queue: &HashMap<u16, Order>) -> [[bool; 2]; 4]{
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


fn cab_order_format_converter(order_queue: &VecDeque<Order>) -> Vec<bool>{
    let mut queue: Vec<bool> = vec![false; 4];
    for order in order_queue {
        queue[*order.get_floor() as usize] = true;
    }
    return queue
}


pub fn assign_hall_orders(last_world_view: WorldView) -> HashMap<u16, VecDeque<Order>> {

    let mut states: HashMap<String, ElevatorState> = HashMap::new();

    let filtered_elevators: HashMap<u16, Elevator> 
    = last_world_view.get_elevator_statuses()
                     .iter()
                     .filter(|(_, elevator)| elevator.get_dead_or_alive() == &DeadOrAlive::Alive)
                     .map(|(id, elevator)| (*id, elevator.clone()))
                     .collect();

    for (id, elevator) in filtered_elevators {
        let id_string: String = id.to_string();
        let cab_requests:Vec<bool> = cab_order_format_converter(elevator.get_cab_requests());
        let state: ElevatorState = ElevatorState::new(elevator.get_behaviour().clone(), 
                                                      elevator.get_floor().clone(), 
                                                      elevator.get_direction().clone(), 
                                                      cab_requests);
        states.insert(id_string, state);
    }

    let filtered_hall_orders: HashMap<u16, Order> 
    = last_world_view.get_hall_order_queue().get_order_queue()
                     .iter()
                     .filter(|(_, order)| order.get_order_status() == &OrderStatus::Confirmed)
                     .map(|(id, order)| (*id, order.clone()))
                     .collect();

    let queue: [[bool; 2]; 4] = hall_order_format_converter(&filtered_hall_orders);
 
    let input: Input = Input::new(queue, states); 
    let path_to_assigner: String = "../Project-resources/cost_fns/hall_request_assigner".to_string(); 

    let assignedHallOrders = match assigner(&input, &path_to_assigner) {
        Ok(assignments) => {assignments}
        Err(_) => {println!("Failed to retrieve assignments from assigner. Constructing empty hashmap.");
                   HashMap::new()}
    };

    return assigner_output_to_assigned_orders(assignedHallOrders, &filtered_hall_orders);

}

pub fn decision_thread(rx_decision: cbc::Receiver<WorldView>, tx_hall_orders: cbc::Sender<Vec<Order>>, my_elevator_id: u16) {

    let mut assigned_hall_orders: HashMap<u16, VecDeque<Order>> = HashMap::new();

    loop {
        match rx_decision.recv(){
            Ok(world_view) => {assigned_hall_orders = assign_hall_orders(world_view)},
            Err(_) => println!("failed to receive on rx_decision for elevator {}", my_elevator_id),
        } 
        let my_filtered_hall_orders: Vec<Order> = assigned_hall_orders.get(&my_elevator_id)
                                                                      .cloned()
                                                                      .unwrap_or_else(VecDeque::new)
                                                                      .into_iter()
                                                                      .collect();
        tx_hall_orders.send(my_filtered_hall_orders).unwrap();
    }
}
