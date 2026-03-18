pub mod
hardware
{
    use std::sync::{Arc, Mutex};
    use std::collections::VecDeque;
    use std::sync::mpsc::{Receiver, Sender};
    use std::thread::{self};
    use std::time::Duration;
    use driver_rust::elevio;
    use crossbeam_channel as cbc;
    use driver_rust::elevio::elev::{DIRN_DOWN, DIRN_STOP, DIRN_UP};
    use driver_rust::elevio::poll::CallButton;

    use crate::message::memory_msg::MemoryData;
    use crate::message::{Message, MessageContent};
    use crate::message::hardware_msg::{ConvertedCallButton, HardwareData};
    use crate::memory::order::{Order, OrderDirection, OrderStatus, OrderType};
    use crate::memory::elevator::ElevatorStatusCommand;

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
        pub next_stop: u8
    }

    impl ElevatorData {
        fn new() -> Self {
            Self {
                current_floor: 0,
                obstruction: false,
                stop_button: false,
                motor_direction: OrderDirection::Up,
                next_stop: 0
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
            loop { 
                // Reset Queue
                let mut order_queue: VecDeque<Order> = VecDeque::new();
                let mut floor_requests: [FloorDirection; ELEV_HEIGHT] = std::array::from_fn(|_| FloorDirection::new());

                if let Ok(cmd) = recv.recv_timeout(Duration::from_millis(10)) {
                    if let MessageContent::Memory(MemoryData{ data: ElevatorStatusCommand::SetCabRequests{elevator_id: _, orders}}) = cmd.data {
                        order_queue = orders;
                    }

                    if order_queue.len() > 0 {
                        order_queue.retain(|order| order.get_order_status() == &OrderStatus::Confirmed);

                        for order in order_queue {
                            match *order.get_order_type() {
                                OrderType::Cab => {
                                    floor_requests[*order.get_floor() as usize].cab = true;
                                },
                                OrderType::Hall => {
                                    if *order.get_direction() == OrderDirection::Up {
                                        floor_requests[*order.get_floor() as usize].up = true;
                                    } else {
                                        floor_requests[*order.get_floor() as usize].down = true;
                                    }
                                }
                            }
                        }

                        for (index, floor) in floor_requests.iter().enumerate().map(|(i, f)| (i as u8, f)) {
                            elevator_command_execute(&elevator, HardwareData::SetCallButtonLight{floor: index, call: 0, status: floor.up});
                            elevator_command_execute(&elevator, HardwareData::SetCallButtonLight{floor: index, call: 1, status: floor.down});
                            elevator_command_execute(&elevator, HardwareData::SetCallButtonLight{floor: index, call: 2, status: floor.cab});
                        }
                    }
                }

                {
                    let mut elevator_data = current_elevator_state.lock().unwrap();
                    let c_floor = elevator_data.current_floor as usize;

                    if floor_requests[c_floor].cab || 
                        (elevator_data.motor_direction == OrderDirection::Up && floor_requests[c_floor].up) ||
                        (elevator_data.motor_direction == OrderDirection::Down && floor_requests[c_floor].down) {
                            elevator_data.next_stop = elevator_data.current_floor;
                    } else {
                        match elevator_data.motor_direction {
                            OrderDirection::Up => {

                            },
                            OrderDirection::Down => {

                            },
                            _ => ()
                        }
                    }

                }

                {
                    let mut elevator_data = current_elevator_state.lock().unwrap();
                    if elevator_data.obstruction || elevator_data.stop_button || elevator_data.current_floor == elevator_data.next_stop {
                        elevator_data.motor_direction = OrderDirection::Stop;
                        elevator_command_execute(&elevator, HardwareData::SetMotorDirection(DIRN_STOP));
                    } else  {
                        let dir = if elevator_data.current_floor < elevator_data.next_stop {DIRN_UP} else {DIRN_DOWN};
                        elevator_command_execute(&elevator, HardwareData::SetMotorDirection(dir));
                    }
                }
            }            
        });
    }

    fn dir_swap() -> OrderDirection{
        OrderDirection::Up
    }

    fn stop_logic() {

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