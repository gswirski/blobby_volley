#![feature(globs)]

extern crate glutin;
extern crate gl;

use std::mem;
use std::ptr;
use std::str;
use gl::types::*;
use glutin::{
    ElementState,
    Event,
    VirtualKeyCode
};

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

static VERTEX_DATA: [GLfloat, ..12] = [
    -0.5, -0.4,
    -0.6, -0.8,
    -0.4, -0.8,

    0.5, -0.4,
    0.4, -0.8,
    0.6, -0.8
];

static VS_SRC: &'static str =
    "#version 120

    attribute vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }";

static FS_SRC: &'static str =
    "#version 120

    void main() {
        gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    }";

fn compile_shader(src: &str, ty: GLuint) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        src.with_c_str(|ptr| gl::ShaderSource(shader, 1, &ptr, ptr::null()));
        gl::CompileShader(shader);

        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(buf.as_slice()).ok().expect("ShaderInfoLog not valid utf8"));
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint { unsafe {
    let program = gl::CreateProgram();
    gl::AttachShader(program, vs);
    gl::AttachShader(program, fs);
    gl::LinkProgram(program);
    // Get the link status
    let mut status = gl::FALSE as GLint;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

    // Fail on error
    if status != (gl::TRUE as GLint) {
        let mut len: GLint = 0;
        gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
        let mut buf = Vec::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
        gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
        panic!("{}", str::from_utf8(buf.as_slice()).ok().expect("ProgramInfoLog not valid utf8"));
    }
    program
} }

fn main() {
    let window = glutin::WindowBuilder::new()
        .with_title("Blobby Volley".to_string())
        .with_dimensions(640, 480)
        .build()
        .unwrap();

    unsafe { window.make_current() };

    gl::load_with(|s| window.get_proc_address(s));

    let mut player_a = Player { x: -0.5, y: -0.8 };
    let player_b = Player { x: 0.5, y: -0.8 };

    let mut vertex_data: Vec<GLfloat> = vec![];
    vertex_data.push_all(player_a.to_vertex_data().as_slice());
    vertex_data.push_all(player_b.to_vertex_data().as_slice());

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       mem::transmute(&vertex_data[0]),
                       gl::DYNAMIC_DRAW);

        let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
        let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
        let shader_program = link_program(vs, fs);

        gl::UseProgram(shader_program);

        let pos_attr = "position".with_c_str(|ptr| gl::GetAttribLocation(shader_program, ptr));
        gl::VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(pos_attr as GLuint);
    }

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

        unsafe {
            vertex_data.clear();
            vertex_data.push_all(player_a.to_vertex_data().as_slice());
            vertex_data.push_all(player_b.to_vertex_data().as_slice());
            gl::BufferData(gl::ARRAY_BUFFER,
                           (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           mem::transmute(&vertex_data[0]),
                           gl::DYNAMIC_DRAW);

            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }

        window.swap_buffers();
    }
}
