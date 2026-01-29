use std::thread;

pub mod udpserver;
pub mod message;

use crate::{message::message::UdpMsg, udpserver::udp_server::Server};

fn main() {

    let network_thread = thread::spawn(move || 
    {
        let mut server: Server = Server::spawn();

        loop
        {
            let huh = "I HAVE NO MOUTH AND I MUST SCREAM\n";
            let data = UdpMsg::new(1, 1, message::message::MsgType::Broadcast, huh.as_bytes().to_vec());

            //server.network_recieve();
            server.network_transmit(data);
        }
    }
    );

    network_thread.join().unwrap();
}
