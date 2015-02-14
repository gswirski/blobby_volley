use geom;
use std::cmp::Ordering::Equal;
use std::num::Float;
use std::collections::HashSet;
use glutin::VirtualKeyCode;

static GRAVITY: f32 = 0.0025;

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

        self.ball.apply_physics();
        //println!("E = {}", self.ball.energy());
    }
}

pub struct Ball {
    pub circle: geom::Circle,
    pub forces: Vec<[f32; 2]>,
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Ball {
        Ball {
            circle: geom::Circle {
                center: geom::Point { x: x, y: y },
                radius: 0.1,
                velocity: geom::Vec2 { x: 0.05, y: 0.04 }
            },
            forces: vec![[0.0, -GRAVITY]],
        }
    }

    fn kin_energy(&self) -> f32 {
        let vx = self.circle.velocity.x;
        let vy = self.circle.velocity.y;
        (vx * vx + vy * vy) * 0.5
    }

    fn pot_energy(&self) -> f32 {
        self.circle.center.y * GRAVITY
    }

    pub fn energy(&self) -> f32 {
        self.kin_energy() + self.pot_energy()
    }

    fn scale_velocity(&mut self, energy: f32) {
        let kin = energy - self.pot_energy();
        let ratio = (kin / self.kin_energy()).sqrt();

        self.circle.velocity.x *= ratio;
        self.circle.velocity.y *= ratio;
    }

    pub fn apply_physics(&mut self) {
        for force in self.forces.iter() {
            self.circle.velocity.x += force[0] / 2.0;
            self.circle.velocity.y += force[1] / 2.0;
        }

        self.circle.center.x += self.circle.velocity.x;
        self.circle.center.y += self.circle.velocity.y;

        for force in self.forces.iter() {
            self.circle.velocity.x += force[0] / 2.0;
            self.circle.velocity.y += force[1] / 2.0;
        }

        let mut particles = vec![
            geom::Line::new(-1.0, -0.93, 1.0, -0.93),
            geom::Line::new(-1.0, -1.0, -1.0, 10000.0),
            geom::Line::new(1.0, -1.0, 1.0, 10000.0),
            geom::Line::new(-0.01, -1.0, -0.01, -0.25),
            geom::Line::new(0.01, -1.0, 0.01, -0.25),
            geom::Line::new(-0.01, -0.25, 0.01, -0.25),
        ];

        particles.sort_by(|a, b|
                          self.circle.distance(&a).partial_cmp(
                              &self.circle.distance(&b)).unwrap_or(Equal)
                          );

        for particle in particles {
            if self.circle.is_intersecting(&particle) {
                let energy = self.energy();
                self.circle.bounce_circle(&particle);
                self.scale_velocity(energy);
                break;
            }
        }
    }

    pub fn _apply_physics(&mut self) {
        for force in self.forces.iter() {
            self.circle.velocity.x += force[0] / 2.0;
            self.circle.velocity.y += force[1] / 2.0;
        }

        self.circle.center.x += self.circle.velocity.x;
        self.circle.center.y += self.circle.velocity.y;

        for force in self.forces.iter() {
            self.circle.velocity.x += force[0] / 2.0;
            self.circle.velocity.y += force[1] / 2.0;
        }

        if self.is_colliding() {
            let energy = self.energy();
            //panic!("exit");
            self.circle.velocity.y *= -1.0;
            self.circle.center.y = -1.66 - self.circle.center.y;
            self.scale_velocity(energy);
        }
    }

    fn is_colliding(&self) -> bool {
        self.circle.center.y < -0.83
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
