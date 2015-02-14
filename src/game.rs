use geom;
use physics;
use std::collections::HashSet;
use std::num::Float;
use glutin::VirtualKeyCode;

pub struct World {
    pub player: Player,
    pub opponent: Player,
    pub ball: Ball,
}

impl World {
    pub fn new() -> World {
        World {
            player: Player::new(-0.5, -0.8),
            opponent: Player::new(0.5, -0.8),
            ball: Ball::new(-0.5, -0.3),
        }
    }

    pub fn tick(&mut self, keys: &HashSet<VirtualKeyCode>) {
        let mut horiz = 0;
        horiz += keys.contains(&VirtualKeyCode::Right) as isize;
        horiz -= keys.contains(&VirtualKeyCode::Left) as isize;

        let jmp = keys.contains(&VirtualKeyCode::Up);

        self.player.calc_velocity(0.02 * horiz as f32, jmp);
        self.player.apply_physics();

        physics::apply(&mut self.ball.circle);
    }
}

pub struct Ball {
    pub circle: geom::Circle,
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Ball {
        Ball {
            circle: geom::Circle {
                center: geom::Point { x: x, y: y },
                radius: 0.1,
                velocity: geom::Vec2 { x: 0.05, y: 0.04 }
            },
        }
    }
}

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub forces: Vec<[f32; 2]>,
    pub velocity: [f32; 2],
}

impl Player {
    pub fn new(x: f32, y: f32) -> Player {
        Player {
            x: x,
            y: y,
            forces: vec![[0.0, -0.0025]],
            velocity: [0.0, 0.0],
        }
    }

    pub fn calc_velocity(&mut self, v_horiz: f32, jmp: bool) {
        self.velocity[0] = v_horiz;

        for force in self.forces.iter() {
            self.velocity[0] += force[0];
            self.velocity[1] += force[1];
        }

        if self.is_on_ground() && jmp {
            self.velocity[1] = 0.06;
        }
    }

    pub fn apply_physics(&mut self) {
        self.x += self.velocity[0];
        self.x = self.x.max(-0.87);
        self.x = self.x.min(-0.14);

        self.y += self.velocity[1];
        self.y = self.y.max(-0.8);
    }

    pub fn is_on_ground(&self) -> bool {
        (self.y + 0.8).abs() < 0.001
    }
}
