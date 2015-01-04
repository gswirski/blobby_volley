#![feature(globs)]

extern crate gl;
extern crate glutin;

mod os;
mod game;
mod renderer;
mod graphics;

fn main() {
    let mut window = os::Window::new("Blobby Volley", 640, 480);
    let context = graphics::load(&window);
    let mut world = game::World::new();

    while !window.should_close {
        window.handle_events();

        world.tick(&window.key_set);

        context.draw_frame(renderer::render(&window, &world));
        window.window.swap_buffers();
    }
}
