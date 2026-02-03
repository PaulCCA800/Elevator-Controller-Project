use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Mutex, Arc};
use std::time;
use std::thread::{self, JoinHandle};

pub mod udpserver;
pub mod message;

use crate::message::message::UdpMsg;
use crate::udpserver::udp_server::Server;

fn main() {
    let mut elevator_threads: Vec<JoinHandle<()>> = vec![];

    let udp_server: Arc<Mutex<Server>> = Arc::new(Mutex::new(Server::spawn()));

    let udp_server_tx = udp_server.clone();
    let udp_server_rx = udp_server.clone();

    let (network_sender, _decision_receiver): (Sender<UdpMsg>, Receiver<UdpMsg>) = mpsc::channel();
    let (_decision_sender, _network_receiver): (Sender<UdpMsg>, Receiver<UdpMsg>) = mpsc::channel();

    // Network Tx Thread
    elevator_threads.push(thread::spawn(move ||
    {
        loop
        {
            {
                let udp_lock = udp_server_tx.lock().unwrap();

                //match network_tx.recv()
                //{
                //    Ok(i)   =>
                //    {
                //        udp_lock.network_transmit(i);
                //    },
                //    Err(_)          => {
                //        ()
                //    }
                //}
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
                    None =>
                    {
                        ()
                    }
                }

            }    
            thread::sleep(time::Duration::from_millis(10));
        }
    }));

    for t in elevator_threads
    {
        t.join().unwrap();
    }
}
