use core::time;
use std::thread;

mod udpserver;
mod misc;
use crate::udpserver::{BACKUP_ADDR, Server};

const PATH: &str = "src/commands.txt";

const MAX_MISSED_MSG: u8 = 10;

fn main() {
    let mut server: Server = Server::init_server();
    
    let mut is_host: bool = false;
    let mut missed_msg_count: u8 = 0;

    let mut txbuf: [u8; 1] = [0; 1];
    let mut rxbuf: [u8; 1] = [0; 1];

    let counting = thread::spawn(move || {
        loop
        {
            if is_host == true
            {
                let mut value = misc::read_from_file(PATH);
                println!("{}", value);
                
                value += 1;
                misc::write_to_file(PATH, value);
            }
            thread::sleep(time::Duration::from_secs(1));
        }
    });

    let backup_ctrl = thread::spawn(move || {
        loop
        {
            if is_host
            {
                Server::server_rebind(&mut server);
                Server::network_transmit(&server, &mut txbuf, &BACKUP_ADDR);
            }
            else
            {
                Server::network_recieve(&server, &mut rxbuf);
                println!("RX: {:?}", rxbuf);
                if rxbuf == [0]
                {
                    missed_msg_count += 1;
                }
                else
                {
                    missed_msg_count = 0;
                }

                if missed_msg_count > MAX_MISSED_MSG
                {
                    is_host = true;
                    misc::spawn_process();
                }
            }

            thread::sleep(time::Duration::from_secs(1));
        }
    });

    counting.join().unwrap();
    backup_ctrl.join().unwrap();
}
