pub mod
Hardware
{
    use driver_rust::elevio::elev::Elevator;

    const LOCAL_ADDR: &str = "localhost:0";

    pub fn
    spawn()
    {
        let elevator = Elevator::init(LOCAL_ADDR, 4).unwrap();
    }
}