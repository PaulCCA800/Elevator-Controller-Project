pub mod
hardware
{
    use std::sync::mpsc::{self, Receiver, Sender};
    use std::thread::{self};
    use std::time::{self, Duration};
    use driver_rust::elevio;
    use crossbeam_channel as cbc;
    use driver_rust::elevio::poll::CallButton;

    use crate::message::message::InternalMsg;

    const LOCAL_ADDR: &str = "localhost:3030";
    const FLOOR_COUNT: u8 = 4;
    const POOL_DUR: Duration = Duration::from_millis(10);

    const DIR_UP    : bool = true;
    const DIR_DOWN  : bool = false;

    struct
    ElevatorData
    {
        target_floor: u8,
        direction: bool,
        stop_at: [u8; 4],
    }
    
    pub fn
    hardware_loop(recv: Receiver<InternalMsg>, send: Sender<InternalMsg>)
    {
        let (call_button_tx, call_button_rx)    = cbc::unbounded::<elevio::poll::CallButton>(); 
        let (floor_sensor_tx, floor_sensor_rx)                  = cbc::unbounded::<u8>(); 
        let (stop_button_tx, stop_button_rx)                = cbc::unbounded::<bool>();
        let (obstruction_tx, obstruction_rx)                = cbc::unbounded::<bool>(); 

        let elevator = elevio::elev::Elevator::init(LOCAL_ADDR, FLOOR_COUNT).unwrap();
        let data = ElevatorData{target_floor: 0, direction: DIR_DOWN, stop_at: [0; 4]};

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


        elevator.motor_direction(elevio::elev::DIRN_UP);

        loop
        {
            cbc::select!{
                recv(call_button_rx) -> o => {
                    call_button_handler(o.unwrap(), &elevator);
                },
                recv(floor_sensor_rx) -> o => {
                    floor_sensor_handler(o.unwrap(), &elevator);
                },
                recv(stop_button_rx) -> o => {
                    stop_button_handler(o.unwrap(), &elevator);
                },
                recv(obstruction_rx) -> o => {
                    obstruction_handler(o.unwrap(), &elevator);
                }
            }

            

            thread::sleep(time::Duration::from_millis(100));
        }
    }

    fn
    call_button_handler(cb: CallButton, elevator: &elevio::elev::Elevator)
    {
        println!("{:?}", cb);
    }

    fn
    floor_sensor_handler(fs: u8, elevator: &elevio::elev::Elevator)
    {
        println!("At floor: {:?}.", fs);
        if fs == 3
        {
            elevator.motor_direction(elevio::elev::DIRN_DOWN);
        }
        else if fs == 0
        {
            elevator.motor_direction(elevio::elev::DIRN_UP);
        }
    }

    fn
    stop_button_handler(sb: bool, elevator: &elevio::elev::Elevator)
    {
        if sb == true
        {
            elevator.motor_direction(elevio::elev::DIRN_STOP);
        }
        println!("Stop Button {:?}", sb);
    }

    fn
    obstruction_handler(ob: bool, elevator: &elevio::elev::Elevator)
    {
        println!("Obstruction {:?}", ob);
    }
}