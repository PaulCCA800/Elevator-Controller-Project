pub mod
hardware
{
    use std::collections::VecDeque;
    use std::sync::mpsc::{Receiver, Sender};
    use std::thread::{self};
    use std::time::Duration;
    use driver_rust::elevio;
    use crossbeam_channel as cbc;
    use driver_rust::elevio::poll::CallButton;

    use crate::message::memory_msg::MemoryData;
    use crate::message::{Message, MessageContent};
    use crate::message::hardware_msg::{ConvertedCallButton, HardwareData};
    use crate::memory::order::{self, Order, OrderStatus, OrderType};
    use crate::memory::elevator::ElevatorStatusCommand;

    const LOCAL_ADDR    : &str = "localhost:15657";
    const FLOOR_COUNT   : u8 = 4;
    const POOL_DUR      : Duration = Duration::from_millis(10);

    struct floor_direction {
        up: bool, 
        down: bool
    }

    pub fn
    hardware_loop(send: Sender<Message>, recv: Receiver<Message>)
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

        thread::spawn(move || 
        {
        loop
        {
            cbc::select!{
                recv(call_button_rx) -> o => {
                    send.send(call_button_handler(o.unwrap())).unwrap();
                },
                recv(floor_sensor_rx) -> o => {
                    send.send(floor_sensor_handler(o.unwrap())).unwrap();
                },
                recv(stop_button_rx) -> o => {
                    send.send(stop_button_handler(o.unwrap())).unwrap();
                },
                recv(obstruction_rx) -> o => {
                    send.send(obstruction_handler(o.unwrap())).unwrap();
                }
            }
        }});

        thread::spawn(move || 
        {

            let mut order_queue: VecDeque<Order> = VecDeque::new();
            let mut machine_id: u64 = 0;



            loop
            { 
                if let Ok(cmd) = recv.recv() {
                    if let MessageContent::Memory(MemoryData{ data: ElevatorStatusCommand::SetCabRequests{elevator_id, orders}}) = cmd.data {
                        machine_id = elevator_id;
                        order_queue = orders;
                    }
                    
                    if order_queue.len() > 0 {
                        order_queue.retain(|order| order.get_order_status() == &OrderStatus::Confirmed);

                        let mut cab_list = order_queue.clone();
                        let mut hall_list = order_queue.clone();

                        cab_list.retain(|call| call.get_order_type() == &OrderType::Cab);
                        hall_list.retain(|hall| hall.get_order_type() == &OrderType::Hall);

                        for cab in cab_list {
                            
                        }

                        for hall in hall_list {

                        }

                    }

                        //elevator_command_execute(&elevator, elevator_cmd);
                }
            }            
        });
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
    elevator_command_execute(elevator: &elevio::elev::Elevator, command: Message)
    {
        match command
        {
            Message{id: _, data: MessageContent::Hardware(content)} => 
            {
                match content
                {
                    HardwareData::SetMotorDirection(dir) =>
                    {
                        elevator.motor_direction(dir);
                    },
                    HardwareData::SetCallButtonLight { floor, call, status } =>
                    {
                        elevator.call_button_light(floor, call, status);
                    },
                    HardwareData::SetDoorLight(status) =>
                    {
                        elevator.door_light(status);
                    }
                    HardwareData::SetStopLight(status) =>
                    {
                        elevator.stop_button_light(status);
                    },
                    HardwareData::SetFloorIndicator(floor) =>
                    {
                        elevator.floor_indicator(floor);
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}