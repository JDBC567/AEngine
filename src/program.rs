use std::ffi::CString;
use crate::shader::Shader;
use crate::vmath::utils::print;

pub struct Program{
    shaders:Vec<Shader>,
    id:gl::types::GLuint,
}

impl Program{
    pub fn new()->Program{
        let mut id_=0;
        unsafe{
            id_=gl::CreateProgram();
        }

        Program{
            shaders:vec![],
            id:id_,
        }
    }

    pub fn attach_shader(&mut self,shader:Shader){
        unsafe{
            gl::AttachShader(self.id,shader.id);
            self.shaders.push(shader);
        }
    }

    pub fn link(&mut self){
        unsafe{
            gl::LinkProgram(self.id);

            let mut s=0;
            gl::GetProgramiv(self.id,gl::LINK_STATUS,&mut s);
            if s==0{
                let mut err=empty_cstring(512);
                gl::GetProgramInfoLog(self.id,512,std::ptr::null_mut(),err.as_ptr() as *mut gl::types::GLchar);
                print(err.to_str().unwrap());
            }

            self.shaders.clear();
        }
    }

    pub fn bind(&self){
        unsafe{
            gl::UseProgram(self.id);
        }
    }
}

fn empty_cstring(size: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(size + 1);
    buffer.extend([b' '].iter().cycle().take(size));
    unsafe { CString::from_vec_unchecked(buffer) }
}