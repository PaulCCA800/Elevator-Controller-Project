use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread::{self, JoinHandle};

mod message;
mod hardware;
mod udpserver;
mod misc;

use crate::message::message::{ElevatorUpdateMsg, ElevatorCommand, UdpMsg};
use crate::udpserver::udp_server::Server;
use crate::misc::{DELAY_DUR};

fn main()
{
    let mut elevator_tasks: Vec<JoinHandle<()>> = Vec::new();

    // Network Section
    let elevator_server: Arc<Mutex<Server>> = Arc::new(Mutex::new(Server::spawn()));
 
    let elevator_server_tx = elevator_server.clone();
    let elevator_server_rx = elevator_server.clone();

    let (network_transmit_src, network_transmit_recv): 
    (Sender<UdpMsg>, Receiver<UdpMsg>) = mpsc::channel();
    let (network_receive_src, network_receive_recv):
    (Sender<UdpMsg>, Receiver<UdpMsg>) = mpsc::channel();

    // Network Tx Thread
    elevator_tasks.push(thread::spawn(move || 
    {
        loop{
            {
                if let Ok(server_lock) = elevator_server_rx.lock(){
                    if let Ok(channel_data) = network_transmit_recv.try_recv(){
                        server_lock.network_transmit(channel_data);
                    }
                }
            }
            thread::sleep(DELAY_DUR);
        }
    }));

    // Network Rx Thread
    elevator_tasks.push(thread::spawn(move || 
    {
        loop{
            {
                if let Ok(mut server_lock) = elevator_server_tx.lock(){
                    server_lock.network_recieve();
                    if let Some(rx_message) = server_lock.get_message(){
                        network_receive_src.send(rx_message).unwrap();
                    }
                }
            }
            thread::sleep(DELAY_DUR);
        }
    }));

    // Hardware Section
    let (hardware_update_src, hardware_update_recv): 
    (Sender<ElevatorUpdateMsg>, Receiver<ElevatorUpdateMsg>) = mpsc::channel();
    let (hardware_command_src, hardware_command_recv):
    (Sender<ElevatorCommand>, Receiver<ElevatorCommand>) = mpsc::channel();

    // Hardware Thread
    elevator_tasks.push(thread::spawn(move || 
    {
        hardware::hardware::hardware_loop(hardware_update_src, hardware_command_recv);
    }));

    for task in elevator_tasks
    {
        task.join().unwrap();
    }
}