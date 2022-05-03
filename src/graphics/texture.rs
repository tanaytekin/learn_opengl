use gl::types::*;
use image::{io::Reader as ImageReader, ColorType};
use std::error::Error;
use std::ffi::c_void;

pub struct Texture {
    texture: GLuint,
    width: u32,
    height: u32,
    format: GLenum,
}

impl Texture {
    pub fn from_path(path: &str) -> Result<Texture, Box<dyn Error>> {
        let img = ImageReader::open(path)?.decode()?;

        let width = img.width();
        let height = img.height();

        let mut texture = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
        let color_type = img.color();

        let format = match color_type {
            ColorType::Rgb8 => gl::RGB,

            ColorType::Rgba8 => gl::RGBA,

            _ => {
                return Err(format!("Unsupported color type: {:?}", color_type).into());
            }
        };

        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as i32,
                width as i32,
                height as i32,
                0,
                format,
                gl::UNSIGNED_BYTE,
                img.as_bytes().as_ptr() as *const c_void,
            )
        };

        Ok(Texture {
            texture,
            width,
            height,
            format,
        })
    }

    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.texture) };
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.texture) };
    }
}
