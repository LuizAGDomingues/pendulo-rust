use vector::Vector;

fn main() {
    println!("Hello, world!");
}

struct Pendulo {
    origin: Vector,
    position: Vector,

    angle: f32,

    angular_velocity: f32,
    angular_acceleration: f32,

    r: f32,
    m: f32,
    g: f32,
}

impl Pendulo {
    fn new(x: f32, y: f32, r: f32) -> Pendulo {
        Pendulo {
            origin: Vector::new(x, y),
            position: Vector,
            angle: ,
            angular_velocity: ,
            angular_acceleration: ,
            r: ,
            m: ,
            g: ,
        }
    }

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

        pub fn add(&mut self, other: Vector) -> &Vector {
            self.x += other.x;
            self.y += other.y;
            self
        }

        pub fn set(&mut self, x: f32, y: f32) {
            self.x = x;
            self.y = y;
        }
    }
}
