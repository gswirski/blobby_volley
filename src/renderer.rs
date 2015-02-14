use os;
use game;
use gl::types::GLfloat;
use std::num::Float;

pub fn render(window: &os::Window, world: &game::World) -> Vec<GLfloat> {
    let mut vertices: Vec<GLfloat> = vec![];
    vertices.push_all(&render_player(&world.player)[..]);
    vertices.push_all(&render_player(&world.opponent)[..]);
    vertices.push_all(&render_ball(&world.ball)[..]);
    vertices.push_all(&render_net()[..]);

    let proportions = window.get_canvas_proportions();
    let mut parity = false;
    vertices.map_in_place(|v| {
        parity = !parity;
        if parity {
            v
        } else {
            (v * proportions) + proportions - 1.0
        }
    })
}

fn render_net() -> Vec<GLfloat> {
    vec![
        -0.01, -1.0,
        -0.01, -0.25,
        0.01, -0.25,
        0.01, -0.25,
        0.01, -1.0,
        -0.01, -1.0,
    ]
}

fn render_player(player: &game::Player) -> Vec<GLfloat> {
    let mut vertices: Vec<GLfloat> = vec![];
    vertices.push_all(&render_circle(player.x, player.y, 0.13)[..]);
    vertices.push_all(&render_circle(player.x, player.y + 0.13, 0.1)[..]);

    vertices
}

fn render_ball(ball: &game::Ball) -> Vec<GLfloat> {
    render_circle(ball.circle.center.x, ball.circle.center.y, ball.circle.radius)
}

fn render_circle(x: f32, y: f32, r: f32) -> Vec<GLfloat> {
    let mut vertex: Vec<GLfloat> = vec![];
    let precision = 400is;
    let tau: f32 = 2.0 * 3.14;
    let startx: f32 = x + (r * (tau / precision as f32).cos());
    let starty: f32 = y + (r * (tau / precision as f32).sin());
    for i in 1..(precision + 1) {
        vertex.push(x);
        vertex.push(y);
        vertex.push(x + (r * (i as f32 * tau / precision as f32).cos()));
        vertex.push(y + (r * (i as f32 * tau / precision as f32).sin()));
        vertex.push(x + (r * ((i + 1) as f32 * tau / precision as f32).cos()));
        vertex.push(y + (r * ((i + 1) as f32 * tau / precision as f32).sin()));
    }
    vertex.pop();
    vertex.pop();
    vertex.push(startx);
    vertex.push(starty);

    vertex
}
