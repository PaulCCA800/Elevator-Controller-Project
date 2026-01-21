use core::time;
use std::{sync::{Arc, Mutex}, thread::{self, JoinHandle}};

mod udpserver;
mod misc;
use crate::udpserver::{BACKUP_ADDR, Server};

const PATH: &str = "src/commands.txt";

const MAX_MISSED_MSG: u8 = 4;

fn main() {
    let server_arc: Arc<Mutex<Server>> = Arc::new(Mutex::new(Server::init_server()));
    
    let mut missed_msg_count: u8 = 0;

    let is_host_arc: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

    let is_host_counting = is_host_arc.clone();
    let is_host_transmit = is_host_arc.clone();

    let mut txbuf: [u8; 1] = [0; 1];
    let mut rxbuf: [u8; 1] = [0; 1];

    let mut threads: Vec<JoinHandle<()>> = vec![];

    threads.push(thread::spawn(move || loop {
        {
            let lock_host = is_host_counting.lock().unwrap();
            if *lock_host == true
            {
                let mut value = misc::read_from_file(PATH);
                println!("{}", value);
                
                value += 1;
                misc::write_to_file(PATH, value);
            }
        }
        thread::sleep(time::Duration::from_secs(1));
    }));

    threads.push(thread::spawn(move || loop {
        {
            let mut lock_host = is_host_transmit.lock().unwrap();
            let mut lock_server = server_arc.lock().unwrap();
            if *lock_host == true
            {
                txbuf[0] = 1;
                lock_server.network_transmit(&mut txbuf, BACKUP_ADDR);
            }
            else
            {
                txbuf[0] = 0;
                lock_server.network_recieve(&mut rxbuf);
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
                    *lock_host = true;
                    lock_server.server_rebind();
                    misc::spawn_process();
                }

                rxbuf[0] = 0;
            }
        }
        thread::sleep(time::Duration::from_millis(100));
    }));

    for i in threads
    {
        i.join().unwrap();
    }
    
}
