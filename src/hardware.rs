pub mod
hardware
{
    use driver_rust::elevio::elev::Elevator;
    use crossbeam_channel as cbc;

    const LOCAL_ADDR: &str = "localhost:0";

    pub struct
    ElevatorData
    {
        elevator: Elevator,

    }

    impl
    ElevatorData
    {
        pub fn
        spawn() -> Self
        {
            Self   
            {
                elevator: Elevator::init(LOCAL_ADDR, 4).unwrap(),
            }
        }

        pub fn
        pool(&mut self)
        {
            
        }
    }
}