use core::time;
use std::thread;

mod udpserver;
mod misc;
use crate::udpserver::Server;

const PATH: &str = "src/commands.txt";

const MAX_MISSED_MSG: u8 = 10;

fn main() {
    let mut is_host: bool = false;
    let mut missed_msg_count: u8 = 0;

    let mut value: u8;

    let r_buf: &mut [u8; 1] = &mut [0; 1];
    let mut t_buf: u8 = 0;

    let sever: Server = Server::init_server();
    
    thread::sleep(time::Duration::from_secs(1));

    //misc::start_backup();

    loop {
        match is_host
        {
            false => {
                if r_buf == &mut [0; 1]
                {
                    println!("Message Lost...");
                    missed_msg_count += 1;
                } else {
                    missed_msg_count = 0;
                }

                if missed_msg_count > MAX_MISSED_MSG
                {
                    is_host = true;
                    misc::start_backup();
                }
            },
            true => {
                value = misc::read_from_file(PATH);
                println!("{value}");

                value += 1;
                misc::write_to_file(PATH, value);

                t_buf = 1;
            },
        }

        sever.update(r_buf, &mut [t_buf]);
        thread::sleep(time::Duration::from_secs(1));
    }
}
