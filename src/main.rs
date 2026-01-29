use std::thread;

pub mod udpserver;
pub mod message;

use crate::udpserver::Server;

fn main() {

    

    let network_thread = thread::spawn(move || 
    {
        let server: Server = Server::spawn();

        loop
        {
            
        }
    }
    );

    let memory_thread = thread::spawn(move || 
    {
        loop
        {

        }
    }
    );

    let hardware_thread = thread::spawn(move || 
    {
        loop
        {

        }
    }
    );

    let decision_thread = thread::spawn(move || 
    {
        loop
        {

        }
    }
    );

    network_thread.join().unwrap();
    memory_thread.join().unwrap();
    hardware_thread.join().unwrap();
    decision_thread.join().unwrap();
}
