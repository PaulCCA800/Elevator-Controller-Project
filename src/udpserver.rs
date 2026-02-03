pub mod
udp_server
{
    use std::net::UdpSocket;

    use crate::message::message::UdpMsg;

    const DEFAULT_ADDR: &str = "0.0.0.0:8080";

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
            let socket = UdpSocket::bind(DEFAULT_ADDR)
                .expect("Failed to initialize UDP server.");

            socket.set_broadcast(true)
                .expect("Failed to set Broadcast");

            socket.set_nonblocking(true)
                .expect("Failed to set non-blocking.");

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
            let mut local_buf_vec: Vec<u8> = Vec::new(); 
            let mut local_buf = [0u8; 1024];
            let recv_status = self.server.recv_from(&mut local_buf);

            local_buf_vec.append(&mut local_buf.to_vec());

            match recv_status
            {
                Ok(recv_tup) =>
                {
                    println!("Recived packet from {}", recv_tup.1);

                    let msg = UdpMsg::decode(local_buf_vec, recv_tup.0);

                    self.recv_queue.push(msg);
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