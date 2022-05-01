pub struct FBuffer {
    pub id: gl::types::GLuint,
    pub len: u32,
}

pub struct IBuffer {
    pub id: gl::types::GLuint,
    pub len: u32,
}

pub const ARRAY_BUFFER: i32 = gl::ARRAY_BUFFER as i32;
pub const ELEMENT_ARRAY_BUFFER: i32 = gl::ELEMENT_ARRAY_BUFFER as i32;
pub const SHADER_STORAGE_BUFFER: i32 = gl::SHADER_STORAGE_BUFFER as i32;

impl FBuffer {
    pub fn new(data: Vec<f32>, buffer_type: i32) -> FBuffer {
        let mut id_ = 0;
        unsafe {
            gl::GenBuffers(1, &mut id_);
            gl::BindBuffer(buffer_type as gl::types::GLenum, id_);
            gl::BufferData(
                buffer_type as gl::types::GLenum,
                (data.len() * 4) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }

        FBuffer {
            id: id_,
            len: data.len() as u32,
        }
    }
}

impl Drop for FBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.id);
        }
    }
}

impl IBuffer {
    pub fn new(data: Vec<i32>, buffer_type: i32) -> IBuffer {
        let mut id_ = 0;
        unsafe {
            gl::GenBuffers(1, &mut id_);
            gl::BindBuffer(buffer_type as gl::types::GLenum, id_);
            gl::BufferData(
                buffer_type as gl::types::GLenum,
                (data.len() * 4) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            )
        }
        IBuffer {
            id: id_,
            len: data.len() as u32,
        }
    }
}

impl Drop for IBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.id);
        }
    }
}
