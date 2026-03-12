use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread::{self, JoinHandle};

mod message;
mod hardware;
mod udpserver;
mod misc;
mod memory;

//use crate::memory::{Elevator, Order};
use crate::message::{Message, MessageContent};
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
    (Sender<Message>, Receiver<Message>) = mpsc::channel();
    let (network_receive_src, network_receive_recv):
    (Sender<Message>, Receiver<Message>) = mpsc::channel();

    // Network Tx Thread
    elevator_tasks.push(thread::spawn(move || 
    {
        loop{
            {
                if let Ok(server_lock) = elevator_server_rx.lock(){
                    if let Ok(channel_data) = network_transmit_recv.try_recv(){
                        let network_data= channel_data.try_into_network().unwrap();
                        server_lock.network_transmit(network_data);                        
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
    (Sender<Message>, Receiver<Message>) = mpsc::channel();
    let (hardware_command_src, hardware_command_recv):
    (Sender<Message>, Receiver<Message>) = mpsc::channel();

    // Hardware Rx Thread
    elevator_tasks.push(thread::spawn(move || 
    {
        hardware::hardware::hardware_loop(hardware_update_src, hardware_command_recv);
    }));

    // Memory Section
    //let states: HashMap<u64, memory::Elevator> = HashMap::new();
    //let states_1: HashMap<u64, Order> = HashMap::new();
    let states_2: HashMap<u64, u8> = HashMap::new();
    //let mut state_matrix: mem::WorldView = mem::WorldView::new(states, states_1, states_2);

    // Placeholders - Replace with other thread sources and add conversion to them
    
    let (_, mem_placeholder_recv):
    (Sender<Message>, Receiver<Message>) = mpsc::channel();

    let (mem_placeholder_src, _):
    (Sender<Message>, Receiver<Message>) = mpsc::channel();

    let id = generate_id();

    elevator_tasks.push(thread::spawn(move || 
    {
        //state_matrix.edit_elevator_status();
        loop{
            {
                while let Ok(command) = mem_placeholder_recv.try_recv(){
                    match command.data
                    {
                        MessageContent::Hardware(_hardware_data) => 
                        {
                            //if let Ok(elevator_command) = hardware_data.hardware_to_memory(id)
                            //{
                            //    state_matrix.edit_elevator_status(elevator_command.data);
                            //}
                        }
                        MessageContent::Network(_network_data) =>
                        {
                            //if let Ok(elevator_command) = network_data.network_to_memory(){
                            //    state_matrix.edit_elevator_status(elevator_command.data);
                            //}
                            
                        },
                        _ => ()
                    }
                }

                //let elevator_data: Elevator = state_matrix.get_elevator(id).clone();
                //match mem_placeholder_src.send(elevator_data)
                //{
                //    Ok(_)   => println!("Transmit Successful, elevator: {id}."),
                //    Err(_)  => println!("Transmit Failed, elevator {id}."),
                //}
            }
            thread::sleep(DELAY_DUR);
        }
    }));

    for task in elevator_tasks
    {
        task.join().unwrap();
    }
}