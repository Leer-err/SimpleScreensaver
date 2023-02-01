use crate::renderer;
use gl::types::*;
use std::ffi::CString;

const SPEED: f32 = 0.2;
const NUM_SECTIONS: usize = 20;
const NUM_POINTS: usize = 100;

#[repr(C)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

pub struct Tunnel {
    program: renderer::Program,
    vbo: GLuint,
    vao: GLuint,
    sections: Vec<Point>,
}

impl Tunnel {
    pub fn new() -> Self {
        let vert_shader = renderer::Shader::from_vert_source(
            &CString::new(include_str!("Shaders/Tunnel.vert")).unwrap(),
        )
        .unwrap();

        let frag_shader = renderer::Shader::from_frag_source(
            &CString::new(include_str!("Shaders/Stars.frag")).unwrap(),
        )
        .unwrap();

        let program = renderer::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

        let oval = Self::init_oval();

        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (oval.len() * std::mem::size_of::<Point>()) as gl::types::GLsizeiptr,
                oval.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
        }

        let mut vao: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,         // index of the generic vertex attribute ("layout (location = 0)")
                2,         // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (std::mem::size_of::<Point>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                std::ptr::null(),                                   // offset of the first component
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Self {
            program,
            vbo,
            vao,
            sections: Vec::with_capacity(NUM_SECTIONS),
        }
    }

    pub fn update(&mut self, time: f32, elapsed: f32) {
        self.program.set_used();

        self.update_sections(time, elapsed);
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::BindVertexArray(self.vao);
        }

        for section in self.sections.iter() {
            unsafe {
                let location =
                    gl::GetUniformLocation(self.program.id(), b"center\0".as_ptr() as *const i8);
                gl::Uniform3f(location, section.x, section.y, section.z);
                gl::DrawArrays(gl::POINTS, 0, NUM_POINTS as i32);
            }
        }

        unsafe {
            gl::BindVertexArray(0);
        }
    }

    fn init_oval() -> Vec<Point> {
        let mut oval = Vec::with_capacity(NUM_SECTIONS);

        for i in 0..NUM_POINTS {
            let x = (std::f32::consts::PI * 2.0 * (i as f32 / NUM_POINTS as f32)).cos() / 2.0;
            let y = (std::f32::consts::PI * 2.0 * (i as f32 / NUM_POINTS as f32)).sin() / 2.0;
            let z = -1.0 * (i as f32 / NUM_POINTS as f32);
            oval.push(Point { x, y, z });
        }

        oval
    }

    fn update_sections(&mut self, time: f32, elapsed: f32) {
        if self.sections.len() < NUM_SECTIONS {
            let x = (elapsed).cos() / 8.0;
            let y = (elapsed).sin() / 8.0;
            let z = -1.0 * ((self.sections.len() + 1) as f32 / NUM_SECTIONS as f32);
            self.sections.push(Point { x, y, z });
        }

        for section in self.sections.iter_mut() {
            section.z += time * SPEED;
            if section.z >= 0.0 {
                let x = (elapsed).cos() / 8.0;
                let y = (elapsed).sin() / 8.0;
                *section = Point { x, y, z: -1.0 };
            }
        }
    }
}
