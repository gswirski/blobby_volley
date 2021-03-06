use geom;
use std::num::Float;
use std::cmp::Ordering::Equal;

static GRAVITY: f32 = 0.0025;

fn kin_energy(ball: &geom::Circle) -> f32 {
    let vx = ball.velocity.x;
    let vy = ball.velocity.y;
    (vx * vx + vy * vy) * 0.5
}

fn pot_energy(ball: &geom::Circle) -> f32 {
    ball.center.y * GRAVITY
}

pub fn energy(ball: &geom::Circle) -> f32 {
    kin_energy(ball) + pot_energy(ball)
}

fn scale_velocity(ball: &mut geom::Circle, energy: f32) {
    let kin = energy - pot_energy(ball);
    let ratio = (kin / kin_energy(ball)).sqrt();

    ball.velocity.x *= ratio;
    ball.velocity.y *= ratio;
}

fn half_apply_gravity(ball: &mut geom::Circle) {
    ball.velocity.y -= GRAVITY / 2.0;
}

pub fn apply(ball: &mut geom::Circle, particles: Vec<geom::Line>) {
    half_apply_gravity(ball);

    ball.center.x += ball.velocity.x;
    ball.center.y += ball.velocity.y;

    half_apply_gravity(ball);

    let mut particles = particles.to_vec();


    let mut changes = true;

    while changes {
        changes = false;

        particles.sort_by(|a, b|
                          ball.distance(&a).partial_cmp(
                              &ball.distance(&b)).unwrap_or(Equal)
                          );

        for particle in &particles {
            if ball.is_intersecting(&particle) {
                let energy = energy(&ball);
                ball.bounce_circle(&particle);
                scale_velocity(ball, energy);
                changes = true;
                break;
            }
        }
    }
}
