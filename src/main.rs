#![feature(globs)]

extern crate glutin;
extern crate gl;

use glutin::{
    ElementState,
    Event,
    VirtualKeyCode
};

mod game;
mod graphics;

fn main() {
    let window = glutin::WindowBuilder::new()
        .with_title("Blobby Volley".to_string())
        .with_dimensions(640, 480)
        .build()
        .unwrap();

    unsafe { window.make_current() };

    let context = graphics::load(&window);
    let mut world = game::World::new();

    let mut right = 0;
    let mut left = 0;

    'main: loop {
        for event in window.poll_events() {
            match event {
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Right)) => right = 1,
                Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Right)) => right = 0,
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Left)) => left = 1,
                Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Left)) => left = 0,
                Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => break 'main,
                _ => ()
            }
        }

        world.move_player(right - left);

        context.draw_frame(&world.to_vertex_data());

        window.swap_buffers();
    }
}
