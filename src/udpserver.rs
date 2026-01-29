pub mod
udp_server
{
    use std::net::UdpSocket;

    use crate::message::message::UdpMsg;

    const DEFAULT_ADDR: &str = "0.0.0.0:0";

    pub struct
    Server
    {
        server      : UdpSocket,
        recv_queue  : Vec<UdpMsg>
    }

    impl 
    Server
    {
        pub fn
        spawn() -> Self
        {
            let socket = UdpSocket::bind("127.0.0.1:0")
                .expect("Failed to initialize UDP server.");

            socket.set_broadcast(true)
                .expect("Failed to set Broadcast");

            socket.set_nonblocking(true)
                .expect("Failed to set non-blocking.");

            //socket.connect("255.255.255.255:8080")
                //.expect("Failed to connect");

            println!("Server started at: {}", socket.local_addr().unwrap());

            Self
            {
                server: socket,
                recv_queue: Vec::new(),
            }
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
            let mut local_buf: Vec<u8> = Vec::new(); 
            let recv_status = self.server.recv(&mut local_buf);

            match recv_status
            {
                Ok(recv_tup) =>
                {
                    println!("Something from {}", recv_tup);
                    self.recv_queue.push(UdpMsg::decode(local_buf, recv_tup));
                },
                Err(_) =>
                    ()
            }
        }
    }
}