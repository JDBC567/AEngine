use crate::shader::Shader;
use crate::vmath::matrix4f::Matrix4f;
use crate::vmath::utils::print;
use std::ffi::CString;

pub struct Program {
    shaders: Vec<Shader>,
    id: gl::types::GLuint,
}

impl Program {
    pub fn new() -> Program {
        let mut id_ = 0;
        unsafe {
            id_ = gl::CreateProgram();
        }

        Program {
            shaders: vec![],
            id: id_,
        }
    }

    pub fn attach_shader(&mut self, shader: Shader) {
        unsafe {
            gl::AttachShader(self.id, shader.id);
            self.shaders.push(shader);
        }
    }

    pub fn link(&mut self) {
        unsafe {
            gl::LinkProgram(self.id);

            let mut s = 0;
            gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut s);
            if s == 0 {
                let mut err = empty_cstring(512);
                gl::GetProgramInfoLog(
                    self.id,
                    512,
                    std::ptr::null_mut(),
                    err.as_ptr() as *mut gl::types::GLchar,
                );
                print(err.to_str().unwrap());
            }

            self.shaders.clear();
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn load_mat4f(&self, mat: Matrix4f, name: &str) {
        unsafe {
            let name = CString::new(name).unwrap();
            gl::UniformMatrix4fv(
                gl::GetProgramResourceLocation(self.id, gl::UNIFORM, name.as_ptr()),
                1,
                gl::FALSE,
                mat.as_ptr(),
            );
        }
    }

    pub fn load_int(&self, val: i32, name: &str) {
        unsafe {
            let name = CString::new(name).unwrap();
            gl::Uniform1i(
                gl::GetProgramResourceLocation(self.id, gl::UNIFORM, name.as_ptr()),
                val,
            );
        }
    }

    pub fn load_float(&self, val: f32, name: &str) {
        unsafe {
            let name = CString::new(name).unwrap();
            gl::Uniform1f(
                gl::GetProgramResourceLocation(self.id, gl::UNIFORM, name.as_ptr()),
                val,
            );
        }
    }
}

fn empty_cstring(size: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(size + 1);
    buffer.extend([b' '].iter().cycle().take(size));
    unsafe { CString::from_vec_unchecked(buffer) }
}
