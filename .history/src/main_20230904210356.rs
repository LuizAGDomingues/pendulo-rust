use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::Window;
use vector::Vector;

fn main() {
    let window = Window::new_centered("Pendulum", (800, 480)).unwrap();
    let win: MyWindowHandler = MyWindowHandler {
        p: Pendulo::new(400.0, 0.0, 200.0),
    };
    window.run_loop(win);
}

struct MyWindowHandler {
    p: Pendulo,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        self.p.update();
    }
}

struct Pendulo {
    origin: vector::Vector,
    position: vector::Vector,

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
            origin: vector::Vector::new(x, y),
            position: vector::Vector::new(0.0, 0.0),
            angle: 1.0,
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            r: r,
            m: 1.0,
            g: 1.5,
        }
    }

    fn update(&mut self) {
        self.angular_acceleration = -1.0 * self.g * self.angle.sin() / self.r;
        self.angular_velocity += self.angular_acceleration;
        self.angle += self.angular_velocity;
        self.position
            .set(self.r * self.angle.sin(), self.r * self.angle.cos());
        self.position.add(&self.origin);
    }

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

        pub fn add(&mut self, other: &Vector) -> &Vector {
            self.x += other.x;
            self.y += other.y;
            self
        }

        pub fn set(&mut self, x: f32, y: f32) -> &Vector {
            self.x = x;
            self.y = y;
            self
        }
    }
}
