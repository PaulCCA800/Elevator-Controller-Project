use std::env;
use std::fmt;
use std::io::{self, Read, Result, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;


#[derive(Clone, Debug)]
pub struct ElevatorHardware {
    addr: String,
    socket: Arc<Mutex<Option<TcpStream>>>,
    pub num_floors: u8,
}


pub const HALL_UP: u8 = 0;
pub const HALL_DOWN: u8 = 1;
pub const CAB: u8 = 2;


pub const DIRN_DOWN: u8 = u8::MAX;
pub const DIRN_STOP: u8 = 0;
pub const DIRN_UP: u8 = 1;


pub fn get_tcp_address() -> String {
    let args: Vec<String> = env::args().collect();

    match args.get(2) {
        Some(arg) => arg.clone(),
        None => "localhost:15657".to_string(),
    }
}


impl ElevatorHardware {

    pub fn init(addr: &str, num_floors: u8) -> Result<ElevatorHardware> {
        let stream = TcpStream::connect(addr)?;
        stream.set_nodelay(true).ok();

        Ok(Self {
            addr: addr.to_string(),
            socket: Arc::new(Mutex::new(Some(stream))),
            num_floors,
        })
    }


    fn reconnect(&self) {
        loop {
            match TcpStream::connect(&self.addr) {
                Ok(stream) => {
                    stream.set_nodelay(true).ok();
                    *self.socket.lock().unwrap() = Some(stream);
                    println!("RECONNECTED TO {}", self.addr);
                    return;
                }
                Err(e) => {
                    println!("RECONNECT FAILED: {}", e);
                    thread::sleep(Duration::from_millis(500));
                }
            }
        }
    }


    fn with_stream<T, F>(&self, mut f: F) -> T
    where
        F: FnMut(&mut TcpStream) -> io::Result<T>,
    {
        loop {
            {
                let mut guard = self.socket.lock().unwrap();

                if guard.is_none() {
                    drop(guard);
                    self.reconnect();
                    continue;
                }

                if let Some(stream) = guard.as_mut() {
                    match f(stream) {
                        Ok(value) => return value,
                        Err(e) => {
                            println!("TCP ERROR: {}. DROPPING CONNECTION.", e);
                            *guard = None;
                        }
                    }
                }
            }

            self.reconnect();
        }
    }


    pub fn motor_direction(&self, dirn: u8) {
        let buf = [1, dirn, 0, 0];
        self.with_stream(|sock| {
            sock.write_all(&buf)?;
            Ok(())
        });
    }


    pub fn call_button_light(&self, floor: u8, call: u8, on: bool) {
        let buf = [2, call, floor, on as u8];
        self.with_stream(|sock| {
            sock.write_all(&buf)?;
            Ok(())
        });
    }


    pub fn floor_indicator(&self, floor: u8) {
        let buf = [3, floor, 0, 0];
        self.with_stream(|sock| {
            sock.write_all(&buf)?;
            Ok(())
        });
    }


    pub fn door_light(&self, on: bool) {
        let buf = [4, on as u8, 0, 0];
        self.with_stream(|sock| {
            sock.write_all(&buf)?;
            Ok(())
        });
    }


    pub fn stop_button_light(&self, on: bool) {
        let buf = [5, on as u8, 0, 0];
        self.with_stream(|sock| {
            sock.write_all(&buf)?;
            Ok(())
        });
    }


    pub fn call_button(&self, floor: u8, call: u8) -> bool {
        let mut buf = [6, call, floor, 0];
        self.with_stream(|sock| {
            sock.write_all(&buf)?;
            sock.read_exact(&mut buf)?;
            Ok(buf[1] != 0)
        })
    }


    pub fn floor_sensor(&self) -> Option<u8> {
        let mut buf = [7, 0, 0, 0];
        self.with_stream(|sock| {
            sock.write_all(&buf)?;
            sock.read_exact(&mut buf)?;
            Ok(if buf[1] != 0 { Some(buf[2]) } else { None })
        })
    }


    pub fn stop_button(&self) -> bool {
        let mut buf = [8, 0, 0, 0];
        self.with_stream(|sock| {
            sock.write_all(&buf)?;
            sock.read_exact(&mut buf)?;
            Ok(buf[1] != 0)
        })
    }


    pub fn obstruction(&self) -> bool {
        let mut buf = [9, 0, 0, 0];
        self.with_stream(|sock| {
            sock.write_all(&buf)?;
            sock.read_exact(&mut buf)?;
            Ok(buf[1] != 0)
        })
    }
}


impl fmt::Display for ElevatorHardware {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let guard = self.socket.lock().unwrap();

        match guard.as_ref() {
            Some(stream) => match stream.peer_addr() {
                Ok(addr) => write!(f, "Elevator@{}({})", addr, self.num_floors),
                Err(_) => write!(f, "Elevator@{}({})", self.addr, self.num_floors),
            },
            None => write!(f, "Elevator@{}({}) [disconnected]", self.addr, self.num_floors),
        }
    }
}