use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Mutex, Arc};
use std::time::{self};
use std::thread::{self, JoinHandle};

mod udpserver;
mod message;
mod hardware;

use crate::message::message::{InternalMsg, UdpMsg};
use crate::udpserver::udp_server::Server;

fn main() {
    let mut elevator_threads: Vec<JoinHandle<()>> = vec![];

    let udp_server: Arc<Mutex<Server>> = Arc::new(Mutex::new(Server::spawn()));

    let udp_server_tx = udp_server.clone();
    let udp_server_rx = udp_server.clone();

    let (network_sender, _decision_receiver): (Sender<UdpMsg>, Receiver<UdpMsg>) = mpsc::channel();
    let (_decision_sender, network_receiver): (Sender<UdpMsg>, Receiver<UdpMsg>) = mpsc::channel();

    let (_decision_sender, elevator_reciever): (Sender<InternalMsg>, Receiver<InternalMsg>) = mpsc::channel();
    let (elevator_data_sender, _decision_elevator_receiver): (Sender<InternalMsg>, Receiver<InternalMsg>) = mpsc::channel();

    // Network Tx Thread
    elevator_threads.push(thread::spawn(move ||
    {
        loop
        {
            {
                let udp_lock = udp_server_tx.lock().unwrap();

                match network_receiver.try_recv()
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
                        network_sender.send(i).unwrap();
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
        hardware::hardware::hardware_loop(elevator_reciever, elevator_data_sender);
    }));

    for t in elevator_threads
    {
        t.join().unwrap();
    }
}
