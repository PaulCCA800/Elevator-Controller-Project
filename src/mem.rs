enum Direction {
    Up,
    Down,
}

struct Order {
    floor: u8,
    cab: bool,
    direction: Direction,
}

struct Elevator {
    id: u8,
    current_floor: u8,
    direction: Direction,
    obstruction: bool,
    stop: bool,
    cab_orders: Vec<Order>,
    hall_orders: Vec<Order>,
}

struct Matrix {
    matrix: Vec<Elevator> 
}

impl Order {
    pub fn new(floor: u8, cab: bool, direction: Direction) -> Self{
        Self{
            floor,
            cab,
            direction,
        }
    }
}

impl Elevator{
    pub fn new(id: u8, current_floor: u8, direction: Direction, 
               obstruction: bool, stop: bool, cab_orders: Vec<Order>, 
               hall_orders: Vec<Order>) -> Self{
        Self{
            id,
            current_floor,
            direction,
            obstruction,
            stop,
            cab_orders,
            hall_orders,
        }
    }
}

impl Matrix {
    pub fn new(matrix: Vec<Elevator>) -> Self{
        Self {matrix}
    }

    fn write_to_matrix(matrix: &mut Matrix, id: u8) {

    }

    fn read_from_matrix(matrix: &Matrix) {

    } 
}

fn send_to_network() {

}

fn recv_from_network() {

}



