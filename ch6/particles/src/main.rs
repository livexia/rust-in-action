extern crate piston_window;

use piston_window::math::{add, mul_scalar, Vec2d};
use piston_window::*;
use rand::prelude::*;

use std::alloc::{GlobalAlloc, Layout, System};

use std::time::Instant;

#[global_allocator]
static ALLOCATOR: ReportingAllocator = ReportingAllocator;

struct ReportingAllocator;

unsafe impl GlobalAlloc for ReportingAllocator {
    // why return *mut u8
    // see: https://old.reddit.com/r/rust/comments/y3t5cf/why_doesnt_globalallocalloc_return_a_usize_pointer/
    // and: https://users.rust-lang.org/t/why-does-alloc-return-u8/55290
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let start = Instant::now();
        let ptr = System.alloc(layout);
        let end = Instant::now();
        let time_taken = end - start;
        let bytes_requested = layout.size();

        eprintln!("{}\t{}", bytes_requested, time_taken.as_nanos());

        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}

struct World {
    curent_turn: u64,
    particles: Vec<Box<Particle>>, // using box to use allocator
    height: f64,
    width: f64,
    rng: ThreadRng,
}

struct Particle {
    height: f64,
    width: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
    color: [f32; 4],
}

impl Particle {
    fn new(world: &World) -> Self {
        let mut rng = thread_rng();
        let x = rng.gen_range(0.0..=world.width);
        let y = world.height;
        let x_velocity = 0.0;
        let y_velocity = rng.gen_range(-2.0..0.0);
        let x_acceleration = 0.0;
        let y_acceleration = rng.gen_range(0.0..0.15);

        Self {
            height: 4.0,
            width: 4.0,
            position: [x, y].into(),
            velocity: [x_velocity, y_velocity].into(),
            acceleration: [x_acceleration, y_acceleration].into(),
            color: [1.0, 1.0, 1.0, 0.99],
        }
    }

    fn update(&mut self) {
        // move Particle to next position
        self.velocity = add(self.velocity, self.acceleration);
        self.position = add(self.position, self.velocity);

        // slow down the acceleration
        self.acceleration = mul_scalar(self.acceleration, 0.7);

        // makes the particle mor transparent over time
        self.color[3] *= 0.95;
    }
}

fn main() {
    let (width, height) = (1280, 960);
    let mut window: PistonWindow = WindowSettings::new("Particles", [width, height])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle(
                [1.0, 0.0, 0.0, 1.0], // red
                [0.0, 0.0, 100.0, 100.0],
                context.transform,
                graphics,
            );
        });
    }
}
