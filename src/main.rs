use std::thread::{self, JoinHandle};

use std::sync::mpsc::{self, Sender, Receiver};

mod hardware;
mod message;

use crate::message::message::{ElevatorUpdateMsg, ElevatorCommand};

fn main()
{
    let mut elevator_tasks: Vec<JoinHandle<()>> = Vec::new();

    // Hardware Channels
    let (hardware_update_src, _hardware_update_recv): 
    (Sender<ElevatorUpdateMsg>, Receiver<ElevatorUpdateMsg>) = mpsc::channel();
    let (_hardware_command_src, hardware_command_recv):
    (Sender<ElevatorCommand>, Receiver<ElevatorCommand>) = mpsc::channel();

    // Hardware Thread
    elevator_tasks.push(thread::spawn(move || 
    {
        hardware::hardware::hardware_loop(hardware_update_src, hardware_command_recv);
    }));

    for task in elevator_tasks
    {
        task.join().unwrap();
    }
}