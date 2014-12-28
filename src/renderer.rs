use game;
use std::num::FloatMath;
use gl::types::GLfloat;

pub fn render(world: &game::World) -> Vec<GLfloat> {
    let mut vertices: Vec<GLfloat> = vec![];
    vertices.push_all(render_player(&world.player).as_slice());
    vertices.push_all(render_player(&world.opponent).as_slice());
    vertices.push_all(render_ball(&world.ball).as_slice());
    vertices.push_all(render_net().as_slice());

    vertices
}

fn render_net() -> Vec<GLfloat> {
    vec![
        -0.01, -1.0,
        -0.01, 0.0,
        0.01, 0.0,
        0.01, 0.0,
        0.01, -1.0,
        -0.01, -1.0,
    ]
}

fn render_player(player: &game::Player) -> Vec<GLfloat> {
    vec![
        player.x, player.y + 0.4,
        player.x - 0.1, player.y,
        player.x + 0.1, player.y,
    ]
}

fn render_ball(ball: &game::Ball) -> Vec<GLfloat> {
    let mut vertex: Vec<GLfloat> = vec![];
    let x = ball.x;
    let y = ball.y;
    let triangle_amount = 400i;
    let twice_pi: f32 = 2.0 * 3.14;
    let radius: f32 = 0.1;
    let startx: f32 = x + (radius * ( twice_pi / triangle_amount as f32).cos());
    let starty: f32 = y + (radius * ( twice_pi / triangle_amount as f32).sin());
    for i in range(1, triangle_amount + 1) {
        vertex.push(x);
        vertex.push(y);
        vertex.push( x + (radius * (i as f32 * twice_pi / triangle_amount as f32).cos()));
        vertex.push( y + (radius * (i as f32 * twice_pi / triangle_amount as f32).sin()));
        vertex.push( x + (radius * ((i + 1) as f32 * twice_pi / triangle_amount as f32).cos()));
        vertex.push( y + (radius * ((i + 1) as f32 * twice_pi / triangle_amount as f32 ).sin()));
    }
    vertex.pop();
    vertex.pop();
    vertex.push(startx);
    vertex.push(starty);

    vertex
}
