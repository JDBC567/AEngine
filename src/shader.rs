use crate::vmath::utils::print;
use std::ffi::CString;
use std::io::Read;

pub struct Shader {
    pub id: gl::types::GLuint,
}

pub const VERTEX_SHADER: i32 = gl::VERTEX_SHADER as i32;
pub const FRAGMENT_SHADER: i32 = gl::FRAGMENT_SHADER as i32;

impl Shader {
    pub fn new(path: &str, shader_type: i32) -> Shader {
        let mut id_ = 0;
        unsafe {
            id_ = gl::CreateShader(shader_type as u32);

            let mut f = std::fs::File::open(path).unwrap();
            let mut code: Vec<u8> = Vec::with_capacity(f.metadata().unwrap().len() as usize + 1);
            f.read_to_end(&mut code).unwrap();

            let shader = std::ffi::CString::from_vec_unchecked(code);

            gl::ShaderSource(id_, 1, &shader.as_ptr(), std::ptr::null());
            gl::CompileShader(id_);

            let mut s = 0;
            gl::GetShaderiv(id_, gl::COMPILE_STATUS, &mut s);
            if s == 0 {
                let mut err = empty_cstring(512);
                gl::GetShaderInfoLog(
                    id_,
                    512,
                    std::ptr::null_mut(),
                    err.as_ptr() as *mut gl::types::GLchar,
                );
                print(err.to_str().unwrap());
            }
        }
        Shader { id: id_ }
    }
}

fn empty_cstring(size: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(size + 1);
    buffer.extend([b' '].iter().cycle().take(size));
    unsafe { CString::from_vec_unchecked(buffer) }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}
