# Distributed Elevator Control System

A distributed elevator control system written in Rust.  
The system uses UDP broadcast for peer-to-peer state sharing and maintains a shared world view across multiple elevator controllers.

## Project Structure

- `src/main.rs` – program entry point
- `src/memory/` – world view, elevator state, orders
- `src/hardware/` – hardware polling and execution
- `src/network/` – UDP broadcast and peer communication
- `src/decision/` – hall order assignment interface

## Requirements

- Rust + Cargo
- Elevator hardware driver / simulator
- Linux environment
- Network support for UDP broadcast

## Build

```bash
cargo build

## Run
```bash
cargo run
