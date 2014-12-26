#![feature(globs)]

extern crate gl;
extern crate glutin;

mod os;
mod game;
mod graphics;

fn main() {
    let mut window = os::Window::new("Blobby Volley", 640, 480);
    let context = graphics::load(&window);
    let mut world = game::World::new();

    'main: while !window.should_close {
        window.handle_events();
        world.move_player(&window.key_set);
        context.draw_frame(&world.to_vertex_data());
        window.window.swap_buffers();
    }
}
