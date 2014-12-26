use gl::types::GLfloat;
use std::collections::HashSet;
use glutin::VirtualKeyCode;

pub struct World {
    player: Player,
    opponent: Player,
}

impl World {
    pub fn new() -> World {
        World {
            player: Player { x: -0.5, y: -0.8 },
            opponent: Player { x: 0.5, y: -0.8 },
        }
    }

    pub fn move_player(&mut self, keys: &HashSet<VirtualKeyCode>) {
        let mut dir = 0;
        dir += keys.contains(&VirtualKeyCode::Right) as int;
        dir -= keys.contains(&VirtualKeyCode::Left) as int;
        self.player.move_p(dir);
    }

    pub fn to_vertex_data(&self) -> Vec<GLfloat> {
        let mut vertex_data: Vec<GLfloat> = vec![];
        vertex_data.push_all(self.player.to_vertex_data().as_slice());
        vertex_data.push_all(self.opponent.to_vertex_data().as_slice());

        vertex_data
    }
}

pub struct Player {
    x: f32,
    y: f32,
}

impl Player {
    pub fn move_p(&mut self, dir: int) {
        self.x += 0.01 * dir as f32;
    }

    pub fn to_vertex_data(&self) -> Vec<GLfloat> {
        return vec![
            self.x, self.y + 0.4,
            self.x - 0.1, self.y,
            self.x + 0.1, self.y,
        ];
    }
}

