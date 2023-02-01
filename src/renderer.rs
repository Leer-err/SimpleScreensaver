use gl::types::*;
use std::ffi::{CStr, CString};

pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(program_id);
        }

        //error handling
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let buffer = vec![b' '; len as usize + 1];
            let error = CString::new(buffer).unwrap();

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id());
            }
        }

        Ok(Program { id: program_id })
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub struct Shader {
    id: u32,
}

impl Shader {
    pub fn from_source(source: &CStr, shader_type: GLenum) -> Result<Shader, String> {
        let shader = shader_from_source(source, shader_type)?;
        Ok(Shader { id: shader })
    }

    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn shader_from_source(source: &CStr, shader_type: GLenum) -> Result<GLuint, String> {
    let shader = unsafe { gl::CreateShader(shader_type) };
    unsafe {
        gl::ShaderSource(shader, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
    }

    let mut success: GLint = 1;
    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: GLint = 0;
        unsafe {
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
        }
        let buffer = vec![b' '; len as usize + 1];
        let error = CString::new(buffer).unwrap();

        unsafe {
            gl::GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut GLchar,
            );
        }
        return Err(error.to_string_lossy().into_owned());
    }
    Ok(shader)
}
