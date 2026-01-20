use std::net::UdpSocket;

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
        let socket = UdpSocket::bind("0.0.0.0:0")
            .expect("Failed to start server");
        
        socket.set_broadcast(true).unwrap();
        socket.broadcast().unwrap();

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
        println!("Transmiting {:?}", transmit_buffer);
        self._server.send(transmit_buffer).unwrap();
    }

    fn
    network_recieve(&self, external_buffer: &mut [u8])
    {
        self._server.recv_from(external_buffer).unwrap();
        println!("Recieved {:?}", external_buffer);
    }

}
