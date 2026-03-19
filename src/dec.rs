use std::collections::{HashMap, VecDeque};
use std::io::Write;
use std::process::{Command, Stdio};
use serde::{Serialize, Deserialize};

//use crate::memory::{Behaviour, DeadOrAlive, Elevator, ElevatorDirection, Order, OrderDirection, OrderStatus, OrderType, WorldView};

use crate::memory::orders::{Order, OrderDirection, OrderStatus, OrderType};
use crate::memory::world_view::{WorldView};
use crate::memory::elevator::{Behaviour, ElevatorDirection, Elevator};

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
    hall_assigner.stdin.as_mut().unwrap().write_all(&json_input);
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
    Ok(assignments)
}


fn assigner_output_to_assigned_orders(orderMap: HashMap<String, Vec<[bool; 2]>>) -> HashMap<u64, VecDeque<Order>>{

    let mut assignedOrdersByID: HashMap<u64, VecDeque<Order>> = HashMap::new(); 
    for (elevator, orders) in orderMap{
        let id: u64 = match elevator.parse() {
            Ok(elevatorID) => elevatorID,
            Err(e) => {print!("Parse failed {}. Returning empty hashmap", e);
                                      return HashMap::new();}
        };
        let mut assignedOrders: VecDeque<Order> = VecDeque::new();  
        let mut floor: u8 = 0;
        for entry in orders {
            if entry[0] == true {
                let up_order: Order = Order::new(floor, OrderType::Hall, 
                                                 OrderDirection::Up);
                assignedOrders.push_back(up_order);
            }
            if entry[1] == true {
                let down_order: Order  = Order::new(floor, OrderType::Hall, 
                                                    OrderDirection::Down);
                assignedOrders.push_back(down_order);
            }
            floor = floor + 1;
        };
        assignedOrdersByID.insert(id, assignedOrders);
    }

    return assignedOrdersByID;
}  


fn hall_order_format_converter(order_queue: &HashMap<u64, Order>) -> [[bool; 2]; 4]{
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
    let mut queue: Vec<bool> = Vec::new();
    for order in order_queue {
        queue[order.get_floor().clone() as usize] = true;
    }
    return queue
}


pub fn assign_hall_orders(last_world_view: WorldView) -> HashMap<u64, VecDeque<Order>> {

    let mut states: HashMap<String, ElevatorState> = HashMap::new();

    let filtered_elevators: HashMap<u64, Elevator> 
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

    let filtered_hall_orders: HashMap<u64, Order> 
    = last_world_view.get_order_queue()
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

    return assigner_output_to_assigned_orders(assignedHallOrders);

}