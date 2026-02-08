use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Mutex, Arc};
use std::time::{self, Duration};
use std::thread::{self, JoinHandle};

mod udpserver;
mod message;
mod hardware;

use crate::message::message::UdpMsg;
use crate::udpserver::udp_server::Server;

use driver_rust::elevio;
use crossbeam_channel as cbc;

const LOCAL_ADDR: &str = "localhost:3030";
const FLOOR_COUNT: u8 = 4;
const POOL_DUR: Duration = Duration::from_millis(10);

fn main() {
    let mut elevator_threads: Vec<JoinHandle<()>> = vec![];

    let udp_server: Arc<Mutex<Server>> = Arc::new(Mutex::new(Server::spawn()));

    let udp_server_tx = udp_server.clone();
    let udp_server_rx = udp_server.clone();

    let (network_sender, _decision_receiver): (Sender<UdpMsg>, Receiver<UdpMsg>) = mpsc::channel();
    let (_decision_sender, network_receiver): (Sender<UdpMsg>, Receiver<UdpMsg>) = mpsc::channel();

    // Network Tx Thread
    elevator_threads.push(thread::spawn(move ||
    {
        loop
        {
            {
                let udp_lock = udp_server_tx.lock().unwrap();

                match network_receiver.try_recv()
                {
                    Ok(i) =>
                    {
                        udp_lock.network_transmit(i);
                    },
                    Err(_) => {()}
                }
            }
            thread::sleep(time::Duration::from_millis(10));
        }
    }));

    // Network Rx Thread
    elevator_threads.push(thread::spawn(move ||
    {
        loop 
        {
            {
                let mut udp_lock = udp_server_rx.lock().unwrap();

                udp_lock.network_recieve();

                match udp_lock.get_message()
                {
                    Some(i) => 
                    {
                        network_sender.send(i).unwrap();
                    },
                    None => {()}
                }

            }    
            thread::sleep(time::Duration::from_millis(10));
        }
    }));

    // Hardware Thread
    elevator_threads.push(thread::spawn(move ||
    {
        let (call_button_tx, call_button_rx)    = cbc::unbounded::<elevio::poll::CallButton>(); 
        let (floor_sensor_tx, floor_sensor_rx)                  = cbc::unbounded::<u8>(); 
        let (stop_button_tx, stop_button_rx)                = cbc::unbounded::<bool>();
        let (obstruction_tx, obstruction_rx)                = cbc::unbounded::<bool>(); 

        let elevator = elevio::elev::Elevator::init(LOCAL_ADDR, FLOOR_COUNT).unwrap();

        {
            let elevator_call = elevator.clone();
            thread::spawn(move || elevio::poll::call_buttons(elevator_call, call_button_tx, POOL_DUR));
        }
        
        {
            let elevator_floor = elevator.clone();
            thread::spawn(move || elevio::poll::floor_sensor(elevator_floor, floor_sensor_tx, POOL_DUR));    
        }

        {
            let elevator_stop = elevator.clone();    
            thread::spawn(move || elevio::poll::stop_button(elevator_stop, stop_button_tx, POOL_DUR));
        }

        {
            let elevator_obstruction = elevator.clone();    
            thread::spawn(move || elevio::poll::obstruction(elevator_obstruction, obstruction_tx, POOL_DUR));
        }

        let mut direction: u8 = elevio::elev::DIRN_UP; 
        elevator.motor_direction(direction);

        loop
        {
            cbc::select!{
                recv(call_button_rx) -> o => {
                    let call_button = o.unwrap();
                    println!("{:?}", call_button);
                },
                recv(floor_sensor_rx) -> o => {
                    let floor_sensor = o.unwrap();
                    println!("{:?}", floor_sensor);
                    if floor_sensor == 3
                    {
                        direction = elevio::elev::DIRN_DOWN;
                    }
                    else if floor_sensor == 0
                    {
                        direction = elevio::elev::DIRN_UP;
                    }
                    elevator.motor_direction(direction);
                },
                recv(stop_button_rx) -> o => {
                    let stop_button = o.unwrap();
                    println!("{:?}", stop_button);
                },
                recv(obstruction_rx) -> o => {
                    let obstruction = o.unwrap();
                    println!("{:?}", obstruction);
                }
            }
            thread::sleep(time::Duration::from_millis(100));
        }
    }));

    for t in elevator_threads
    {
        t.join().unwrap();
    }
}
