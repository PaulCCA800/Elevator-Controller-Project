pub mod
udp_server
{
    use std::{net::UdpSocket, net::SocketAddr};

    use crate::message::message::UdpMsg;

    const DEFAULT_ADDR  : &str = "0.0.0.0:8080";
    const EMPTY_ADDR    : &str = "0.0.0.0:0";
    const DUMMY_CONNECT : &str = "192.168.0.1:40000";

    pub struct
    Server
    {
        server      : UdpSocket,
        local_addr  : SocketAddr,
        recv_queue  : Vec<UdpMsg>
    }

    impl 
    Server
    {
        pub fn
        spawn() -> Self
        {
            let socket = UdpSocket::bind(DEFAULT_ADDR)
                .expect("Failed to initialize UDP server.");

            socket.set_broadcast(true)
                .expect("Failed to set Broadcast");

            socket.set_nonblocking(true)
                .expect("Failed to set non-blocking.");

            println!("Server started at: {}", socket.local_addr().unwrap());

            Self
            {
                server      : socket,
                local_addr  : Server::get_local_ip(),
                recv_queue  : Vec::new(),
            }
        }

        fn
        get_local_ip() -> SocketAddr
        {
            let temp_local_ip = UdpSocket::bind(EMPTY_ADDR).unwrap();
            temp_local_ip.connect(DUMMY_CONNECT).unwrap();
            temp_local_ip.local_addr().unwrap()
        }

        pub fn
        network_transmit(&self, mut message: UdpMsg)
        {
            let transmit_buffer = message.encode();

            let a = self.server.send_to(&transmit_buffer, "255.255.255.255:8080");
            match a {
                Ok(_) => (),
                Err(e) => println!("Error: {e}")
            };
        }

        pub fn
        network_recieve(&mut self)
        {
            let mut local_buf_vec: Vec<u8> = Vec::new(); 
            let mut local_buf = [0u8; 1024];
            let recv_status = self.server.recv_from(&mut local_buf);

            local_buf_vec.append(&mut local_buf.to_vec());

            match recv_status
            {
                Ok(recv_tup) =>
                {
                    // Filter self sent packets
                    if recv_tup.1.ip() != self.local_addr.ip()
                    {
                        println!("Recived packet from {}", recv_tup.1);
                    
                        let msg = UdpMsg::decode(local_buf_vec, recv_tup.0);

                        match msg
                        {
                            Some(data) => {self.recv_queue.push(data);},
                            None => ()
                        }         
                    }
                },
                Err(_) =>
                    ()
            }
        }

        pub fn
        get_message(&mut self) -> Option<UdpMsg>
        {      
            self.recv_queue.pop()
        }
    }
}