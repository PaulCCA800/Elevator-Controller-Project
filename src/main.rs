use crossbeam_channel as cbc;
use std::thread;

mod decision;
mod memory;
mod elevator_id;
mod udpnet;
mod elevator_driver;

use crate::elevator_id::generate_id;
use crate::elevator_driver::hardware_execution::{hardware_output_thread, spawn_hardware_input_threads};
use crate::elevator_driver::elev::ElevatorHardware;
use crate::memory::elevator::Elevator;
use crate::memory::orders::Order;
use crate::memory::world_view::{WorldView, MemoryCommand, memory_thread};
use crate::decision::decision_thread;
use crate::udpnet::udp_execution::{network_rx_thread, network_tx_thread};

fn main() {
    let my_elevator_id: u16 = generate_id();
    let my_session_id: u16 = rand::random();
    let udp_port: u16 = 20013;

    let elevator_hw = ElevatorHardware::init("localhost:15657", 4).unwrap();

    let (tx_memory, rx_memory) = cbc::unbounded::<MemoryCommand>();
    let (tx_network_to_memory, rx_network_to_memory) = cbc::unbounded::<WorldView>();

    let (tx_elevator_state, rx_elevator_state) = cbc::unbounded::<Elevator>();
    let (tx_decision, rx_decision) = cbc::unbounded::<WorldView>();
    let (tx_hall_orders, rx_hall_orders) = cbc::unbounded::<Vec<Order>>();

    let (tx_network_tx, rx_network_tx) = cbc::unbounded::<WorldView>();

    spawn_hardware_input_threads(elevator_hw.clone(), tx_memory.clone(), my_elevator_id);

    {
        let tx = tx_network_to_memory.clone();
        thread::spawn(move || {
            network_rx_thread(udp_port, tx);
        });
    }

    {
        thread::spawn(move || {
            network_tx_thread(udp_port, rx_network_tx);
        });
    }

    {
        let tx_elevator_state = tx_elevator_state.clone();
        let tx_decision = tx_decision.clone();
        let tx_network_tx = tx_network_tx.clone();
        thread::spawn(move || {
            memory_thread(
                my_elevator_id,
                my_session_id,
                rx_memory,
                rx_network_to_memory,
                tx_elevator_state,
                tx_decision,
                tx_network_tx,
            );
        });
    }

    {
        let tx_hall_orders_thread = tx_hall_orders.clone();
        thread::spawn(move || {
            decision_thread(
                rx_decision,
                tx_hall_orders_thread,
                my_elevator_id,
            );
        });
    }

    {
        let tx_memory_thread = tx_memory.clone();
        thread::spawn(move || {
            hardware_output_thread(
                elevator_hw,
                rx_elevator_state,
                rx_hall_orders,
                tx_memory_thread,
                my_elevator_id,
            );
        });
    }

    loop {
        thread::park();
    }
}
