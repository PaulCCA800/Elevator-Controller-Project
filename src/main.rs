use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread::{self, JoinHandle, sleep};

mod message;
mod hardware;
mod udpserver;
mod misc;
mod mem;

use crate::mem::{Elevator};
use crate::message::message::{ElevatorUpdateMsg, ElevatorCommand, UdpMsg, MatrixCmd};
use crate::udpserver::udp_server::Server;
use crate::misc::{DELAY_DUR, generate_id};

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
                        println!("Received Message: {:?}", rx_message);
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

    // Hardware Rx Thread
    elevator_tasks.push(thread::spawn(move || 
    {
        hardware::hardware::hardware_loop(hardware_update_src, hardware_command_recv);
    }));

    // Memory Section
    let states: HashMap<u64, mem::Elevator> = HashMap::new();
    let mut state_matrix: mem::Matrix = mem::Matrix::new(states);

    // Placeholders - Replace with other thread sources and add conversion to them
    
    let (_, mem_placeholder_recv):
    (Sender<MatrixCmd>, Receiver<MatrixCmd>) = mpsc::channel();

    let (mem_placeholder_src, _):
    (Sender<Elevator>, Receiver<Elevator>) = mpsc::channel();

    let id = generate_id();

    elevator_tasks.push(thread::spawn(move || 
    {
        loop{
            {
                while let Ok(command) = mem_placeholder_recv.try_recv(){
                    state_matrix.edit_matrix(command);
                }

                let elevator_data: Elevator = state_matrix.get(id).clone();
                match mem_placeholder_src.send(elevator_data)
                {
                    Ok(_)   => println!("Transmit Successful, elevator: {id}."),
                    Err(_)  => println!("Transmit Failed, elevator {id}."),
                }
            }
            thread::sleep(DELAY_DUR);
        }
    }));

    elevator_tasks.push(thread::spawn(move || 
    {
        let mut num: u16 = 0;
        
        loop
        {
            {
                num += 1;
                let mut data: Vec<u8> = Vec::new();
                data.push(1);
                data.push(2);
                data.push(3);
                data.push(4);
                let msg = UdpMsg::new(0, num, message::message::MsgType::Broadcast, data);
                network_transmit_src.send(msg).unwrap();
            }
            thread::sleep(DELAY_DUR);
            thread::sleep(DELAY_DUR);
            thread::sleep(DELAY_DUR);
            thread::sleep(DELAY_DUR);
            thread::sleep(DELAY_DUR);
        }
    }));

    for task in elevator_tasks
    {
        task.join().unwrap();
    }
}