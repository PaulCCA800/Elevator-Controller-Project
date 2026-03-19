pub mod
udp_server
{
    use std::sync::{Arc, Mutex, mpsc::{Receiver, Sender}};
    use std::thread;
    use crate::misc::DELAY_DUR;

    use crate::message::Message;

    use std::{net::UdpSocket, net::SocketAddr};

    const DEFAULT_ADDR  : &str = "0.0.0.0:8080";
    const EMPTY_ADDR    : &str = "0.0.0.0:0";
    const DUMMY_CONNECT : &str = "192.168.0.1:40000";

    pub struct
    Server
    {
        server      : UdpSocket,
        local_addr  : SocketAddr,
        recv_queue  : Vec<Message>
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
        network_transmit(&self, message: Message)
        {
            let transmit_buffer: Vec<u8> = bincode::serialize(&message).unwrap();
            
            if let Err(error) = self.server.send_to(&transmit_buffer, "255.255.255.255:8080"){
                println!("Transmit Error: {error}");
            }
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
                    
                        if let Ok(msg) = bincode::deserialize(&local_buf_vec){
                            self.recv_queue.push(msg);
                        }
                    }
                },
                Err(_) =>
                    ()
            }
        }

        pub fn
        get_message(&mut self) -> Option<Message>
        {      
            self.recv_queue.pop()
        }

        pub fn spawn_tx_thread(recv: Receiver<Message>, server: Arc<Mutex<Server>>) {
            loop{
                {
                    if let Ok(server_lock) = server.lock(){
                        if let Ok(channel_data) = recv.try_recv(){
                            let network_data= channel_data.try_into_network().unwrap();
                            server_lock.network_transmit(network_data);                        
                        }
                    }
                }
                thread::sleep(DELAY_DUR);
            }
        }

        pub fn spawn_rx_thread(src: Sender<Message>, server: Arc<Mutex<Server>>) {
            loop{
                {
                    if let Ok(mut server_lock) = server.lock(){
                        server_lock.network_recieve();
                        if let Some(rx_message) = server_lock.get_message(){
                            src.send(rx_message).unwrap();
                        }
                    }
                }
                thread::sleep(DELAY_DUR);
            }
        }
    }
}