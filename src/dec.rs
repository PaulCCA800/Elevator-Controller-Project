use std::collections::HashMap;
use std::io::Write;
use std::process::{Command, Stdio};
use serde::{Serialize, Deserialize};

use crate::mem::{WorldView, Elevator, Order, ElevatorStatusCommand, OrderQueueCommand};


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Behaviour {
    Idle,
    Moving,
    DoorOpen,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Direction {
    Up,
    Down,
    Stop,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElevatorState {
    behaviour: Behaviour,
    floor: u8,
    direction: Direction,
    cab_requests: Vec<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    hall_orders: Vec<[bool; 2]>,
    states: HashMap<String, ElevatorState>,
}

pub type Output = HashMap<String, Vec<[bool; 2]>>;

pub fn assigner(input: &Input, exe_path: &String) -> anyhow::Result<Output> {
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

pub fn make_elevator_decision(lastWorldView: WorldView) -> ElevatorStatusCommand {
    
}