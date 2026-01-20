use std::{net::UdpSocket, time::Duration};
use std::str;

pub struct 
Server
{
    _server: UdpSocket,
    _role: bool,
}

impl Server
{
    pub fn
    init_server() -> Self
    {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        
        println!("UDP Server Started at {:?}", socket.local_addr().unwrap());

        socket.set_broadcast(true).unwrap();
        println!("Broadcast set to: {:?}", socket.broadcast().unwrap());
        socket.set_nonblocking(true).unwrap();
        socket.set_read_timeout(Some(Duration::new(1, 0))).unwrap();

        Self{
            _server: socket,
            _role: false,
        }
        
    }

    pub fn
    update(&self, recieve_buffer: &mut [u8], transmit_buffer: &mut [u8])
    {
        self.network_recieve(recieve_buffer);
        self.network_broadcast(transmit_buffer);
    }

    fn
    network_broadcast(&self, transmit_buffer: &mut [u8])
    {        
        self._server.send_to(transmit_buffer, self._server.local_addr().unwrap()).unwrap();
        self._server.broadcast().unwrap();

        if transmit_buffer != [0; 100]
        {
            println!("Transmiting {:?}", transmit_buffer);
        }
    }

    fn
    network_recieve(&self, external_buffer: &mut [u8])
    {
        match self._server.recv_from(external_buffer)
        {
            Ok(_) => None,
            Err(_) => Some(1)
        };
        
        if external_buffer != [0; 100]
        {
            let s = str::from_utf8(external_buffer).unwrap();
            println!("Recieved {:?}", s);
        }
    }

}
