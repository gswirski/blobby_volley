use std::collections::HashSet;
use glutin::VirtualKeyCode;

pub struct World {
    pub player: Player,
    pub opponent: Player,
    pub ball: Ball,
}

impl World {
    pub fn new() -> World {
        World {
            player: Player { x: -0.5, y: -0.8 },
            opponent: Player { x: 0.5, y: -0.8 },
            ball: Ball { x: -0.5, y: 0.0 },
        }
    }

    pub fn move_player(&mut self, keys: &HashSet<VirtualKeyCode>) {
        let mut dir = 0;
        dir += keys.contains(&VirtualKeyCode::Right) as int;
        dir -= keys.contains(&VirtualKeyCode::Left) as int;
        self.player.move_p(dir);
    }
}

pub struct Ball {
    pub x: f32,
    pub y: f32,
}

pub struct Player {
    pub x: f32,
    pub y: f32,
}

impl Player {
    pub fn move_p(&mut self, dir: int) {
        self.x += 0.01 * dir as f32;
    }
}
