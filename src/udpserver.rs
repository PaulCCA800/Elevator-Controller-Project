use std::net::UdpSocket;
use std::str;

pub const BACKUP_ADDR: &str = "127.0.0.1:3000";
pub const PRODUC_ADDR: &str = "0.0.0.0:8000";

pub struct 
Server
{
    _server: UdpSocket,
}

impl Server
{
    pub fn
    init_server() -> Self
    {
        let socket = UdpSocket::bind(BACKUP_ADDR).unwrap(); 
        println!("UDP Server Started at {:?}", socket.local_addr().unwrap());
        socket.set_nonblocking(true).unwrap();

        Self{
            _server: socket,
        }
        
    }

    pub fn
    server_rebind(&mut self)
    {
        if self._server.local_addr().unwrap().to_string() == BACKUP_ADDR
        {
            self._server = UdpSocket::bind(PRODUC_ADDR).unwrap();
        }
    }

    pub fn
    network_transmit(&self, transmit_buffer: &mut [u8], target: &str)
    {        
        match self._server.send_to(transmit_buffer, target)
        {
            Ok(i) =>
            {
                println!("Transmited to backup, {} bytes", i);
            }
            Err(e) =>
            {
                println!("Error! {}", e);
            }
        }
    }

    pub fn
    network_recieve(&self, external_buffer: &mut [u8])
    {
        
        match self._server.recv_from(external_buffer)
        {
            Ok((size, src)) => 
            {
                println!("Packet from: {}, and size: {}", src, size);
                self.network_transmit(&mut [1], src.to_string().as_mut_str());
                
            },
            Err(e) =>
            {
                println!("Error! {}", e);
            },
        };
    }

}
