use crate::renderer;
use gl::types::*;
use std::ffi::CString;

const SCALE: f32 = 5.0;
const NUM_STARS: usize = 100;

#[repr(C)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    fn new() -> Self {
        let x = rand::random::<f32>() * 2.0 - 1.0;
        let y = rand::random::<f32>() * 2.0 - 1.0;
        let z = -1.0;
        Self { x, y, z }
    }
}

pub struct Starfield {
    program: renderer::Program,
    vbo: GLuint,
    vao: GLuint,
    stars: Vec<Point>,
}

impl Starfield {
    pub fn new() -> Self {
        let vert_shader = renderer::Shader::from_vert_source(
            &CString::new(include_str!("Shaders/Stars.vert")).unwrap(),
        )
        .unwrap();

        let frag_shader = renderer::Shader::from_frag_source(
            &CString::new(include_str!("Shaders/Stars.frag")).unwrap(),
        )
        .unwrap();

        let program = renderer::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

        let mut stars = Self::init_stars();

        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (stars.len() * std::mem::size_of::<Point>()) as gl::types::GLsizeiptr,
                stars.as_ptr() as *const gl::types::GLvoid,
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
                3,         // the number of components per generic vertex attribute
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
            stars,
        }
    }

    pub fn update(&mut self, time: f32) {
        self.program.set_used();

        self.update_stars(time);
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (self.stars.len() * std::mem::size_of::<Point>()) as gl::types::GLsizeiptr,
                self.stars.as_ptr() as *const gl::types::GLvoid,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::POINTS, 0, self.stars.len() as i32);
            gl::BindVertexArray(0);
        }
    }

    fn init_stars() -> Vec<Point> {
        let mut stars = Vec::with_capacity(NUM_STARS);

        for i in 0..NUM_STARS {
            let mut point = Point::new();
            point.z = -rand::random::<f32>();
            stars.push(point);
        }

        stars
    }

    fn update_stars(&mut self, time: f32) {
        for star in self.stars.iter_mut() {
            star.z += time / SCALE;
            if star.x.abs() > star.z.abs() || star.y.abs() > star.z.abs() {
                *star = Point::new();
            }
        }
    }
}
