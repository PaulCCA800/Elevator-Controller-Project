pub mod
hardware
{
    use std::sync::{Arc, Mutex};
    use std::collections::VecDeque;
    use std::sync::mpsc::{Receiver, Sender};
    use std::thread::{self};
    use std::time::{Duration, Instant};
    use driver_rust::elevio;
    use crossbeam_channel as cbc;
    use driver_rust::elevio::elev::{DIRN_DOWN, DIRN_STOP, DIRN_UP};
    use driver_rust::elevio::poll::CallButton;

    use crate::message::memory_msg::MemoryData;
    use crate::message::{Message, MessageContent};
    use crate::message::hardware_msg::{ConvertedCallButton, HardwareData};
    use crate::memory::orders::{Order, OrderDirection, OrderStatus, OrderType};
    use crate::memory::world_view::ElevatorStatusCommand;

    const LOCAL_ADDR    : &str = "localhost:15657";
    const FLOOR_COUNT   : u8 = 4;
    const ELEV_HEIGHT   : usize = 4;
    const POOL_DUR      : Duration = Duration::from_millis(10);

    #[derive(Debug)]
    struct FloorDirection {
        pub up: bool, 
        pub down: bool, 
        pub cab: bool
    }

    impl FloorDirection {
        pub fn new() -> Self {
            Self { up: false, down: false, cab: false }
        }
    }

    struct ElevatorData {
        pub current_floor: u8,
        pub obstruction: bool,
        pub stop_button: bool,
        pub motor_direction: OrderDirection,
        pub next_stop: u8,
        pub door_status: bool,
        pub door_timeout: Option<Instant>
    }

    impl ElevatorData {
        fn new() -> Self {
            Self {
                current_floor: 0,
                obstruction: false,
                stop_button: false,
                motor_direction: OrderDirection::Up,
                next_stop: 0,
                door_status: false,
                door_timeout: None
            }
        }
    }

    pub fn
    hardware_loop(send: Sender<Message>, recv: Receiver<Message>)
    {
        let current_elevator: Arc<Mutex<ElevatorData>> = Arc::new(Mutex::new(ElevatorData::new()));

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

        let current_elevator_sens = current_elevator.clone();
        thread::spawn(move || 
        {
        loop
        {
            cbc::select!{
                recv(call_button_rx) -> o => {
                    send.send(call_button_handler(o.unwrap())).unwrap();
                },
                recv(floor_sensor_rx) -> o => {
                    {
                        let mut floor_lock = current_elevator_sens.lock().unwrap();
                        floor_lock.current_floor = o.unwrap();
                    }
                    send.send(floor_sensor_handler(o.unwrap())).unwrap();

                },
                recv(stop_button_rx) -> o => {
                    {
                        let mut floor_lock = current_elevator_sens.lock().unwrap();
                        floor_lock.stop_button = o.unwrap();
                    }
                    send.send(stop_button_handler(o.unwrap())).unwrap();
                },
                recv(obstruction_rx) -> o => {
                    {
                        let mut floor_lock = current_elevator_sens.lock().unwrap();
                        floor_lock.obstruction = o.unwrap();
                    }
                    send.send(obstruction_handler(o.unwrap())).unwrap();
                }
            }
        }});

let current_elevator_state = current_elevator.clone();
        thread::spawn(move || 
        {
            let mut order_queue: VecDeque<Order> = VecDeque::new();
            let mut floor_requests: [FloorDirection; ELEV_HEIGHT] = std::array::from_fn(|_| FloorDirection::new());

            loop { 
                if let Ok(cmd) = recv.recv_timeout(Duration::from_millis(10)) {
                    if let MessageContent::Memory(MemoryData{ data: ElevatorStatusCommand::SetCabRequests{elevator_id: _, orders}}) = cmd.data {
                        order_queue = orders;
                        
                        floor_requests = std::array::from_fn(|_| FloorDirection::new());

                        if order_queue.len() > 0 {
                            order_queue.retain(|order| order.get_order_status() == &OrderStatus::Confirmed);

                            for order in &order_queue {
                                let f = *order.get_floor() as usize;
                                match *order.get_order_type() {
                                    OrderType::Cab => floor_requests[f].cab = true,
                                    OrderType::Hall => {
                                        if *order.get_direction() == OrderDirection::Up {
                                            floor_requests[f].up = true;
                                        } else {
                                            floor_requests[f].down = true;
                                        }
                                    }
                                }
                            }

                            for i in 0..ELEV_HEIGHT {
                                elevator_command_execute(&elevator, HardwareData::SetCallButtonLight{floor: i as u8, call: 0, status: floor_requests[i].up});
                                elevator_command_execute(&elevator, HardwareData::SetCallButtonLight{floor: i as u8, call: 1, status: floor_requests[i].down});
                                elevator_command_execute(&elevator, HardwareData::SetCallButtonLight{floor: i as u8, call: 2, status: floor_requests[i].cab});
                            }
                        }
                    }
                }

                {
                    let mut elevator_data = current_elevator_state.lock().unwrap();
                    let c_floor = elevator_data.current_floor as usize;
                    elevator_command_execute(&elevator, HardwareData::SetFloorIndicator(c_floor as u8));

                    if elevator_data.door_status {
                        if elevator_data.obstruction {
                            elevator_data.door_timeout = Some(std::time::Instant::now());
                        }

                        if let Some(timer) = elevator_data.door_timeout {
                            if timer.elapsed() >= Duration::from_secs(3) {
                                elevator_data.door_status = false;
                                elevator_data.door_timeout = None;
                                elevator_command_execute(&elevator, HardwareData::SetDoorLight(false));
                            }
                        }
                        elevator_command_execute(&elevator, HardwareData::SetMotorDirection(DIRN_STOP));
                    } 
                    else if elevator_data.current_floor == elevator_data.next_stop {
                        let req = &floor_requests[c_floor];
                        let should_stop = req.cab || 
                                          (elevator_data.motor_direction == OrderDirection::Up && req.up) || 
                                          (elevator_data.motor_direction == OrderDirection::Down && req.down) ||
                                          (elevator_data.motor_direction == OrderDirection::Stop);

                        if should_stop {
                            stop_logic(&mut elevator_data, &elevator);
                        } else {
                            elevator_data.motor_direction = dir_swap(elevator_data.motor_direction);
                        }
                    } 
                    else {
                        if !elevator_data.stop_button {
                            let dir = if elevator_data.current_floor < elevator_data.next_stop { DIRN_UP } else { DIRN_DOWN };
                            elevator_command_execute(&elevator, HardwareData::SetMotorDirection(dir));
                        } else {
                            elevator_command_execute(&elevator, HardwareData::SetMotorDirection(DIRN_STOP));
                        }
                    }

                    if !elevator_data.door_status {
                        let c_floor = elevator_data.current_floor as usize;
                        match elevator_data.motor_direction {
                            OrderDirection::Up => {
                                if let Some(target) = ((c_floor + 1)..ELEV_HEIGHT).find(|&f| floor_requests[f].up || floor_requests[f].cab || floor_requests[f].down) {
                                    elevator_data.next_stop = target as u8;
                                } else if let Some(target) = (0..ELEV_HEIGHT).rev().find(|&f| floor_requests[f].down || floor_requests[f].cab || floor_requests[f].up) {
                                    elevator_data.next_stop = target as u8;
                                    if elevator_data.next_stop < elevator_data.current_floor {
                                        elevator_data.motor_direction = OrderDirection::Down;
                                    }
                                }
                            },
                            OrderDirection::Down => {
                                if let Some(target) = (0..c_floor).rev().find(|&f| floor_requests[f].down || floor_requests[f].cab || floor_requests[f].up) {
                                    elevator_data.next_stop = target as u8;
                                } else if let Some(target) = (0..ELEV_HEIGHT).find(|&f| floor_requests[f].up || floor_requests[f].cab || floor_requests[f].down) {
                                    elevator_data.next_stop = target as u8;
                                    if elevator_data.next_stop > elevator_data.current_floor {
                                        elevator_data.motor_direction = OrderDirection::Up;
                                    }
                                }
                            },
                            OrderDirection::Stop => {
                                if let Some(target) = (0..ELEV_HEIGHT).find(|&f| floor_requests[f].up || floor_requests[f].down || floor_requests[f].cab) {
                                    elevator_data.next_stop = target as u8;
                                    elevator_data.motor_direction = if elevator_data.next_stop > elevator_data.current_floor { OrderDirection::Up } else { OrderDirection::Down };
                                }
                            }
                        }
                    }
                }
            }            
        });
    }

    fn dir_swap(current_dir: OrderDirection) -> OrderDirection {
        match current_dir {
            OrderDirection::Up => OrderDirection::Down,
            OrderDirection::Down => OrderDirection::Up,
            _ => OrderDirection::Up,
        }
    }

    fn stop_logic(elevator_data: &mut ElevatorData, elevator: &elevio::elev::Elevator) {
        elevator_data.motor_direction = OrderDirection::Stop;
        elevator.motor_direction(DIRN_STOP);
        elevator_data.door_status = true;
        elevator.door_light(true);
        elevator_data.door_timeout = Some(std::time::Instant::now());
    }

    fn
    call_button_handler(cb: CallButton) -> Message
    {
        Message::new(
            MessageContent::Hardware(
                HardwareData::CallButton (ConvertedCallButton::from_call_button(cb))
            ))
    }

    fn
    floor_sensor_handler(fs: u8) -> Message
    {
        Message::new(
            MessageContent::Hardware(
                HardwareData::FloorSensor(fs)
            )
        )
    }

    fn
    stop_button_handler(sb: bool) -> Message
    {
        Message::new(
            MessageContent::Hardware(
                HardwareData::StopButton(sb)
            )
        )
    }

    fn
    obstruction_handler(ob: bool) -> Message
    {
        Message::new(
            MessageContent::Hardware(
                HardwareData::Obstruction(ob)
            )
        )
    }

    fn
    elevator_command_execute(elevator: &elevio::elev::Elevator, command: HardwareData)
    {
        match command
        {
            HardwareData::SetMotorDirection(dir) => {
                elevator.motor_direction(dir);
            },
            HardwareData::SetCallButtonLight { floor, call, status } => {
                elevator.call_button_light(floor, call, status); 
            },
            HardwareData::SetDoorLight(status) => {
                elevator.door_light(status);
            }
            HardwareData::SetStopLight(status) => {
                elevator.stop_button_light(status);
            },
            HardwareData::SetFloorIndicator(floor) => {
                elevator.floor_indicator(floor);
            }
            _ => (),
        }
    }
}