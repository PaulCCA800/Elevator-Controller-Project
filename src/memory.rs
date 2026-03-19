use std::sync::mpsc::{Receiver, Sender};

use crate::message::Message;

pub mod elevator;
pub mod order;
pub mod world_view;

pub fn spawn_memory_thread(
    network_transmit_src: Sender<Message>,
    hardware_command_src: Sender<Message>,
    network_receive_recv: Receiver<Message>,
    hardware_update_recv: Receiver<Message>) {
    
}