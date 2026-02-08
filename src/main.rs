use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Mutex, Arc};
use std::time;
use std::thread::{self, JoinHandle};

pub mod udpserver;
pub mod message;
mod mem;
mod misc;

use crate::mem::{Matrix, MatrixCmd, Elevator};
use crate::message::message::UdpMsg;
use crate::message::message::InternalMsg
use crate::misc::generate_id;
use crate::udpserver::udp_server::Server;

fn main() {

    let id: u64 = generate_id();

    let states: HashMap<u64, mem::Elevator> = HashMap::new();
    let mut state_matrix: mem::Matrix =  Matrix::new(states);

    let mut elevator_threads: Vec<JoinHandle<()>> = vec![];

    let udp_server: Arc<Mutex<Server>> = Arc::new(Mutex::new(Server::spawn()));

    let udp_server_tx = udp_server.clone();
    let udp_server_rx = udp_server.clone();

    let (net_to_mem_tx, mem_from_net_rx): 
    (Sender<UdpMsg>, Receiver<UdpMsg>) = mpsc::channel();

    let (mem_to_net_tx, net_from_mem_rx): 
    (Sender<UdpMsg>, Receiver<UdpMsg>) = mpsc::channel();

    let (hw_to_mem_tx, mem_from_hw_rx): 
    (Sender<MatrixCmd>, Receiver<MatrixCmd>) = mpsc::channel();

    let (mem_to_hw_tx, hw_from_mem_rx): 
    (Sender<Elevator>, Receiver<Elevator>) = mpsc::channel();
    
    // Network Tx Thread
    elevator_threads.push(thread::spawn(move ||
    {
        loop
        {
            {
                let udp_lock = udp_server_tx.lock().unwrap();

                match mem_from_net_rx.try_recv()
                {
                    Ok(i) => 
                    {
                        udp_lock.network_transmit(i);
                    },
                    Err(_) => 
                    {
                        ()
                    }
                }
            }
            thread::sleep(time::Duration::from_millis(10));
        }
    }));

    // Network Rx Thread
    elevator_threads.push(thread::spawn(move ||
    {
        loop 
        {
            {
                let mut udp_lock = udp_server_rx.lock().unwrap();

                udp_lock.network_recieve();

                match udp_lock.get_message()
                {
                    Some(i) => 
                    {
                        net_to_mem_tx.send(i).unwrap();
                    },
                    None =>
                    {
                        ()
                    }
                }

            }    
            thread::sleep(time::Duration::from_millis(10));
        }
    }));

    // Memory Thread
    elevator_threads.push(thread::spawn(move || {
        loop{  
            while let Ok(c) = mem_from_hw_rx.try_recv() {
                state_matrix.edit_matrix(c);
            }

            let this_elevator: Elevator = state_matrix.get(id).clone();
            match mem_to_hw_tx.send(this_elevator){
                Ok(()) => println!("Successful transmit from memory to hardware, elevator: {}", id),
                Err(_) => println!("Failed to transmit from memory to hardware, elevator: {}", id)
            }

            thread::sleep(time::Duration::from_millis(500));
        }
    }));

    for t in elevator_threads
    {
        t.join().unwrap();
    }
}
