use crate::aengine::Engine;
use crate::texture::{Texture, RGB8};
use crate::vmath::utils::print;
use crate::vmath::vector2::Vector2;

pub struct FrameBuffer {
    id: gl::types::GLuint,
    pub textures: Vec<Texture>,
}

pub const FBO_COLOR0: i32 = gl::COLOR_ATTACHMENT0 as i32;

impl FrameBuffer {
    pub fn new(size: Vector2<i32>) -> FrameBuffer {
        let mut id_ = 0;
        let tex = Texture::new_empty(RGB8, size);
        unsafe {
            gl::GenFramebuffers(1, &mut id_);
            gl::BindFramebuffer(gl::FRAMEBUFFER, id_);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                tex.id,
                0,
            );
            Engine::get_error();
            if (gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE) {
                print("aaaaaa");
            }
        }
        FrameBuffer {
            id: id_,
            textures: vec![tex],
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}
