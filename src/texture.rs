use crate::vmath::vector2::Vector2;
use image::GenericImageView;
use std::ptr::null;

pub struct Texture {
    pub id: gl::types::GLuint,
}

pub const RGB8: i32 = gl::RGB8 as i32;

impl Texture {
    pub fn new(path: &str, tex_type: i32) -> Texture {
        let mut id_ = 0;
        unsafe {
            gl::GenTextures(1, &mut id_);
            gl::BindTexture(gl::TEXTURE_2D, id_);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::NEAREST as gl::types::GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::NEAREST as gl::types::GLint,
            );
            let img = image::open(&std::path::Path::new(path))
                .expect(format!("failed to load texture {}", path).as_str());
            let data = img.raw_pixels();
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                tex_type,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                &data[0] as *const u8 as *const std::ffi::c_void,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        Texture { id: id_ }
    }

    pub fn new_empty(tex_type: i32, size: Vector2<i32>) -> Texture {
        let mut id_ = 0;
        unsafe {
            gl::GenTextures(1, &mut id_);
            gl::BindTexture(gl::TEXTURE_2D, id_);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::NEAREST as gl::types::GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::NEAREST as gl::types::GLint,
            );

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                tex_type,
                size.x,
                size.y,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                null(),
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        Texture { id: id_ }
    }

    pub fn bind(&self, level: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + level);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.id);
        }
    }
}
