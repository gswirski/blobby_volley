#![feature(globs)]

extern crate glutin;
extern crate gl;

use gl::types::GLfloat;
use glutin::{
    ElementState,
    Event,
    VirtualKeyCode
};

mod graphics;

struct Player {
    x: f32,
    y: f32,
}

impl Player {
    fn move_p(&mut self, dir: int) {
        self.x += 0.01 * dir as f32;
    }

    fn to_vertex_data(&self) -> Vec<GLfloat> {
        return vec![
            self.x, self.y + 0.4,
            self.x - 0.1, self.y,
            self.x + 0.1, self.y,
        ];
    }
}


fn main() {
    let window = glutin::WindowBuilder::new()
        .with_title("Blobby Volley".to_string())
        .with_dimensions(640, 480)
        .build()
        .unwrap();

    unsafe { window.make_current() };

    let context = graphics::load(&window);

    let mut player_a = Player { x: -0.5, y: -0.8 };
    let player_b = Player { x: 0.5, y: -0.8 };

    let mut vertex_data: Vec<GLfloat> = vec![];
    vertex_data.push_all(player_a.to_vertex_data().as_slice());
    vertex_data.push_all(player_b.to_vertex_data().as_slice());

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

        player_a.move_p(right - left);

        vertex_data.clear();
        vertex_data.push_all(player_a.to_vertex_data().as_slice());
        vertex_data.push_all(player_b.to_vertex_data().as_slice());

        context.draw_frame(&vertex_data);

        window.swap_buffers();
    }
}
