fn main() {
    println!("Hello, world!");
}

struct Pendulo {
    angle: f32,

    angular_velocity: f32,
    angular_acceleration: f32,

    r: f32,
    m: f32,
    g: f32,
}

impl Pendulo {
    fn new() {}

    fn update() {}

    fn draw() {}
}

mod vector {

    pub struct Vector {
        pub x: f32,
        pub y: f32,
    }

    impl Vector {
        pub fn new(x: f32, y: f32) -> Vector {
            Vector { x, y }
        }

        pub fn add() {}

        pub fn set() {}
    }
}
