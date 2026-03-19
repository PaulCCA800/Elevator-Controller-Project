use crate::memory::world_view::{WorldView};
use crate::udpnet::bcast::{tx, rx};

use crossbeam_channel as cbc;

pub fn network_tx_thread(
    port: u16,
    rx_network_tx: cbc::Receiver<WorldView>,
) {
    if let Err(e) = tx(port, rx_network_tx) {
        eprintln!("network_tx_thread failed: {}", e);
    }
}


pub fn network_rx_thread(
    port: u16,
    tx_network_to_memory: cbc::Sender<WorldView>,
) {
    if let Err(e) = rx(port, tx_network_to_memory) {
        eprintln!("network_rx_thread failed: {}", e);
    }
}