extern crate gl;
extern crate sdl2;

pub mod renderer;
pub mod starfield;
pub mod tunnel;
pub mod window;

use starfield::Starfield;
use std::time::Instant;
use tunnel::Tunnel;

pub fn main() {
    let mut window = window::Window::new(500, 500);

    let mut stars = Tunnel::new();

    let mut frame = Instant::now();
    let start = Instant::now();
    window.run(&mut || {
        let prev_frame = frame;
        frame = Instant::now();
        let duration = frame.duration_since(prev_frame);

        stars.update(duration.as_secs_f32(), start.elapsed().as_secs_f32());
    });
}
