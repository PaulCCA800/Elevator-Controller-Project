pub mod
hardware
{
    use std::sync::mpsc::{Receiver, Sender};
    use std::thread::{self};
    use std::time::{self, Duration};
    use driver_rust::elevio;
    use crossbeam_channel as cbc;
    use driver_rust::elevio::poll::CallButton;

    use crate::message::{Message, MessageContent};
    use crate::message::hardware_msg::{ConvertedCallButton, HardwareData};

    const LOCAL_ADDR    : &str = "localhost:15657";
    const FLOOR_COUNT   : u8 = 4;
    const POOL_DUR      : Duration = Duration::from_millis(10);
    
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

            thread::sleep(time::Duration::from_millis(100));
        }});

        thread::spawn(move || 
        {
            loop
            {
                while let Ok(cmd) = recv.try_recv() {
                    elevator_command_execute(&elevator, cmd);           
                }
                thread::sleep(time::Duration::from_millis(10));
            }            
        });
    }

    fn
    call_button_handler(cb: CallButton) -> Message
    {
        Message::new_local(
            MessageContent::Hardware(
                HardwareData::DataCallButton { 
                    call_button_data: ConvertedCallButton::from_call_button(cb) 
                }
            ))
    }

    fn
    floor_sensor_handler(fs: u8) -> Message
    {
        Message::new_local(
            MessageContent::Hardware(
                HardwareData::DataFloorSensor { floor: fs }
            )
        )
    }

    fn
    stop_button_handler(sb: bool) -> Message
    {
        Message::new_local(
            MessageContent::Hardware(
                HardwareData::DataStopButton { status: sb }
            )
        )
    }

    fn
    obstruction_handler(ob: bool) -> Message
    {
        Message::new_local(
            MessageContent::Hardware(
                HardwareData::DataObstruction { status: ob }
            )
        )
    }

    fn
    elevator_command_execute(elevator: &elevio::elev::Elevator, command: Message)
    {
        match command
        {
            Message{id, data: MessageContent::Hardware(content)} => 
            {
                match content
                {
                    HardwareData::SetMotorDirection { dir } =>
                    {
                        elevator.motor_direction(dir);
                    },
                    HardwareData::SetCallButtonLight { floor, call, status } =>
                    {
                        elevator.call_button_light(floor, call, status);
                    },
                    HardwareData::SetDoorLight { status } =>
                    {
                        elevator.door_light(status);
                    }
                    HardwareData::SetStopLight { status } =>
                    {
                        elevator.stop_button_light(status);
                    },
                    HardwareData::SetFloorIndicator { floor } =>
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