extern crate gl;

use os;
use gl::types::*;
use std::mem;
use std::ptr;
use std::str;
use std::num::FloatMath;

pub struct Context {
    pub vao: u32,
    pub vbo: u32,
}

pub fn load(window: &os::Window) -> Context {
    gl::load(&window.window);

    let (mut vao, mut vbo) = (0, 0);

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
        let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
        let shader_program = link_program(vs, fs);

        gl::UseProgram(shader_program);

        let pos_attr = "position".with_c_str(|ptr| gl::GetAttribLocation(shader_program, ptr));
        gl::VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(pos_attr as GLuint);
    }

    Context { vao: vao, vbo: vbo }
}

fn ball_vec() -> Vec<GLfloat> {
    let mut vertex: Vec<GLfloat> = vec![];
    let triangle_amount = 400i;
    let twice_pi: f32 = 2.0 * 3.14;
    let radius: f32 = 0.1;
    let x: f32 = -0.4;
    let y: f32 = -0.4;
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
    return vertex;
}

impl Context {
    pub fn draw_frame(&self, data: & Vec<GLfloat>) {
        unsafe {
            let mut vertex: Vec<GLfloat> = vec![];
            vertex.push_all(data.as_slice());
            vertex.push_all(ball_vec().as_slice());
            gl::BufferData(gl::ARRAY_BUFFER,
                           (vertex.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           mem::transmute(&vertex[0]),
                           gl::DYNAMIC_DRAW);

            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::DrawArrays(gl::TRIANGLES, 0, vertex.len() as i32);

        }
    }
}

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
