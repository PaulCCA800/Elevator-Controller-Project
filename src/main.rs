mod udp_srv;
//mod tcp_srv;

fn main() {
    let _ = udp_srv::start_server();
    //let _ = tcp_srv::start_server();
}
