use std::env;
use std::thread;
use std::time::Duration;

mod udpserver;
mod misc;
use crate::udpserver::Server;

const PATH: &str = "src/commands.txt";

fn main() {
    let mut is_host: bool = false;

    let mut value: u8 = 0;

    let r_buf: &mut [u8; 100] = &mut [0; 100];
    let t_buf: &mut [u8; 100] = &mut [0; 100];

    let sever: Server = Server::init_server();
    
    loop {
        match is_host
        {
            false => {

            },
            true => {
                println!("{value}");

                value += 1;
                misc::write_to_file(PATH, value);
            },
        }

        sever.update(r_buf, t_buf);
    }
}
