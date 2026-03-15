use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread::{self, JoinHandle};

mod message;
mod hardware;
mod network;
mod misc;
mod memory;

use crate::memory::spawn_memory_thread;
use crate::message::{Message};
use crate::network::udp_server::Server;

fn main()
{
    let mut elevator_tasks: Vec<JoinHandle<()>> = Vec::new();

    // Network Section
    let elevator_server: Arc<Mutex<Server>> = Arc::new(Mutex::new(Server::spawn()));
 
    let elevator_server_tx = elevator_server.clone();
    let elevator_server_rx = elevator_server.clone();

    // Network Channels
    let (network_transmit_src, network_transmit_recv): 
    (Sender<Message>, Receiver<Message>) = mpsc::channel();
    let (network_receive_src, network_receive_recv):
    (Sender<Message>, Receiver<Message>) = mpsc::channel();

    // Hardware Channels
    let (hardware_command_src, hardware_command_recv):
    (Sender<Message>, Receiver<Message>) = mpsc::channel();
    let (hardware_update_src, hardware_update_recv): 
    (Sender<Message>, Receiver<Message>) = mpsc::channel();
    

    // Network Tx Thread
    elevator_tasks.push(thread::spawn(move || {
        Server::spawn_tx_thread(network_transmit_recv, elevator_server_tx);
    }));

    // Network Rx Thread
    elevator_tasks.push(thread::spawn(move || {
        Server::spawn_rx_thread(network_receive_src, elevator_server_rx);
    }));

    // Hardware Thread
    elevator_tasks.push(thread::spawn(move || {
        hardware::hardware::hardware_loop(hardware_update_src, hardware_command_recv);
    }));

    // Memory Section
    elevator_tasks.push(thread::spawn(move || {
        spawn_memory_thread(network_transmit_src, hardware_command_src, network_receive_recv, hardware_update_recv);
    }));

    for task in elevator_tasks
    {
        task.join().unwrap();
    }
}