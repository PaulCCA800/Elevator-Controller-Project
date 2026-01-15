use std::net::UdpSocket;

pub fn
start_server()
{
    let socket = UdpSocket::bind("0.0.0.0:34254").unwrap();
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
