use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Mutex, Arc};
use std::time::{self};
use std::thread::{self, JoinHandle};

use crate::message::message::{ElevatorUpdateMsg, UdpMsg, ElevatorCommand};
mod hardware;
mod udpserver;
mod message;
mod mem;
mod misc;

use crate::mem::{Matrix, MatrixCmd, Elevator};
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

    let (network_sender, _decision_receiver): 
    (Sender<UdpMsg>, Receiver<UdpMsg>) = mpsc::channel();
    
    let (_decision_sender_to_net, network_receiver): 
    (Sender<UdpMsg>, Receiver<UdpMsg>) = mpsc::channel();

    let (decision_sender_elev_command, elevator_reciever): 
    (Sender<ElevatorCommand>, Receiver<ElevatorCommand>) = mpsc::channel();
    
    let (elevator_data_sender, _decision_elevator_receiver):
    (Sender<ElevatorUpdateMsg>, Receiver<ElevatorUpdateMsg>) = mpsc::channel();
    
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
                    Err(_) => {()}
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
                    None => {()}
                }

            }    
            thread::sleep(time::Duration::from_millis(10));
        }
    }));

    // Hardware Thread
    elevator_threads.push(thread::spawn(move ||
    {
        hardware::hardware::hardware_loop(elevator_data_sender, elevator_reciever);
    }));


    // Testing Thread
    elevator_threads.push(thread::spawn(move || 
    {
        loop
        {
            let e = _decision_elevator_receiver.try_recv();
            match e
            {
                Ok(a) => println!("{:?}", a),
                Err(_) => ()
            }

            
            decision_sender_elev_command.send(ElevatorCommand::StopLightSet(true)).unwrap();
            thread::sleep(time::Duration::from_millis(100));
            decision_sender_elev_command.send(ElevatorCommand::StopLightSet(false)).unwrap();
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
