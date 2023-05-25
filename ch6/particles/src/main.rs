extern crate piston_window;

use piston_window::math::{add, mul_scalar, Vec2d};
use piston_window::*;
use rand::prelude::*;

use std::alloc::{GlobalAlloc, Layout, System};

use std::time::Instant;

// this attribute run on macos getting illegal hardware instruction
// #[global_allocator]
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

impl World {
    fn new(width: f64, height: f64) -> Self {
        Self {
            curent_turn: 0,
            particles: Vec::new(),
            height,
            width,
            rng: thread_rng(),
        }
    }

    fn add_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let particle = Particle::new(self);
            let boxed_particel = Box::new(particle);
            self.particles.push(boxed_particel);
        }
    }

    fn remove_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let mut to_delete = None;

            // with book's code, this always remove first particel(oldest)
            for (i, particle) in self.particles.iter().enumerate() {
                if particle.color[3] < 0.02 {
                    to_delete = Some(i);
                }
                break; // bug?
            }

            if let Some(i) = to_delete {
                self.particles.remove(i);
            } else {
                self.particles.remove(0);
            }
        }
    }

    fn update(&mut self) {
        let n = self.rng.gen_range(-3..=3);

        if n > 0 {
            self.add_shapes(n);
        } else {
            self.remove_shapes(n);
        }

        // dealloc
        self.particles.shrink_to_fit();

        for shape in &mut self.particles {
            shape.update();
        }

        self.curent_turn += 1;
    }
}

fn main() {
    let (width, height) = (1280.0, 960.0);
    let mut window: PistonWindow = WindowSettings::new("Particles", [width, height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut world = World::new(width, height);
    world.add_shapes(1000);

    while let Some(event) = window.next() {
        world.update();

        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.15, 0.17, 0.17, 0.9], graphics);
            rectangle(
                [1.0, 0.0, 0.0, 1.0], // red
                [0.0, 0.0, 100.0, 100.0],
                context.transform,
                graphics,
            );
        });
    }
}
