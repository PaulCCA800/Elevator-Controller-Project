use std::fmt::Error;
use std::net::UdpSocket;
use std::str;

use crate::message;

const DEFAULT_ADDR: &str = "0.0.0.0:0";

pub struct 
Server
{
    server: UdpSocket,
    recv_queue: Vec<u8>
}

impl Server
{
    pub fn
    spawn() -> Self
    {
        let socket = UdpSocket::bind(DEFAULT_ADDR).unwrap(); 
        println!("UDP Server Started at {:?}", socket.local_addr().unwrap());
        socket.set_nonblocking(true).unwrap();

        Self{
            server: socket,
            recv_queue: Vec::new(),
        }
    }

    pub fn
    network_transmit(&self, message: message::Msg)
    {        
        let transmit_msg = message.convert_msg();
        self.server.send(&transmit_msg)
            .expect("Network transmit failed.");
    }

    pub fn
    network_recieve(&mut self) -> message::Msg
    {
        let _ = self.server.recv_from(&mut self.recv_queue)
            .expect("Failed to receive data");

        let recv = self.decode().expect("Message Decode Failed");
        
        self.recv_queue.clear();

        let src = &recv[0];
        let sync = &recv[1];
        let data = &recv[2];

        message::Msg::new(String::from(src), sync.parse::<u8>().unwrap(), String::from(data)) 
    }

    fn
    decode(&self) -> Result<Vec<String>, Error>
    {
        let mut temp: Vec<String> = Vec::new();
        temp.push(String::from("reee"));
        temp.push(String::from("reee"));
        temp.push(String::from("reee"));
        Ok(temp)
    }

}
