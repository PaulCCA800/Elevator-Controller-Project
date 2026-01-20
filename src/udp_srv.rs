use std::{net::UdpSocket, time::Duration};

pub fn
start_server()
{
    let socket = UdpSocket::bind("0.0.0.0:4000").unwrap();
    let _ = socket.set_read_timeout(Some(Duration::new(1, 0)));
    let _ = socket.set_broadcast(true).unwrap();

}

pub fn
start_new_server()
{
    let socket = UdpSocket::bind("0.0.0.0:4000").unwrap();
    let mut buffer = [0; 1024];

    loop
    {
        let (size, source) = socket.recv_from(&mut buffer).unwrap();
        let request = String::from_utf8_lossy(&buffer[..size]);

        println!("Recived {} from {}.", request, source);

        let response = "Hello from server!";

        socket.send_to(response.as_bytes(), source).unwrap();
    }
}
