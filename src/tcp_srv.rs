use std::net::TcpListener;

pub fn
start_server()
{
    let listener = TcpListener::bind("0.0.0.0:4000").unwrap();

    for stream in listener.incoming()
    {
        let stream = stream.unwrap();

        println!("New connection! {:?}", stream);
    }
}
