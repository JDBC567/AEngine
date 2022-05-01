use crate::buffer::{FBuffer, IBuffer};

pub struct Mesh {
    ibuffers: Vec<IBuffer>,
    fbuffers: Vec<FBuffer>,
    ia: bool,
    size: i32,
    pub id: gl::types::GLuint,
}

impl Mesh {
    pub fn new() -> Mesh {
        let mut id_ = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id_);
        }

        Mesh {
            ibuffers: vec![],
            fbuffers: vec![],
            ia: false,
            id: id_,
            size: 0,
        }
    }

    pub fn setup(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn finish_setup(&self) {
        unsafe {
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    pub fn attach_fbuffer(&mut self, buffer: FBuffer, size: u8) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, buffer.id);
            let mut index = 0;
            if self.ia {
                index = (self.fbuffers.len() + self.ibuffers.len() - 1) as gl::types::GLuint;
            } else {
                index = (self.fbuffers.len() + self.ibuffers.len()) as gl::types::GLuint;
                self.size = buffer.len as i32 / size as i32;
            }
            gl::EnableVertexAttribArray(index);
            gl::VertexAttribPointer(
                index,
                size as gl::types::GLint,
                gl::FLOAT,
                gl::FALSE,
                0,
                0 as *const gl::types::GLvoid,
            );
        }

        self.fbuffers.push(buffer);
    }

    pub fn attach_ibuffer(&mut self, buffer: IBuffer, size: u8) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, buffer.id);
            let mut index = 0;
            if self.ia {
                index = (self.fbuffers.len() + self.ibuffers.len() - 1) as gl::types::GLuint;
            } else {
                index = (self.fbuffers.len() + self.ibuffers.len()) as gl::types::GLuint;
                self.size = buffer.len as i32 / size as i32;
            }
            gl::EnableVertexAttribArray(index);
            gl::VertexAttribPointer(
                index,
                size as gl::types::GLint,
                gl::INT,
                gl::FALSE,
                0,
                0 as *const gl::types::GLvoid,
            );

            self.ibuffers.push(buffer);
        }
    }

    pub fn attach_indices(&mut self, buffer: IBuffer) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer.id);
        }
        self.size = buffer.len as i32;
        self.ibuffers.push(buffer);
        self.ia = true;
    }

    pub fn draw_arrays(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
            gl::DrawArrays(gl::TRIANGLES, 0, self.size);
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
            gl::DrawElements(
                gl::TRIANGLES,
                self.size,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid,
            );
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.id);
        }
        self.ibuffers.clear();
        self.fbuffers.clear();
    }
}
