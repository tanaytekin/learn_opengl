use std::ffi::CStr;

pub fn gl_str_to_rust_string(name: u32) -> String {
    unsafe {
        let buf = gl::GetString(name);
        let c_str = CStr::from_ptr(buf as *mut i8);
        let string = c_str.to_str().unwrap().to_owned();
        string
    }
}

mod shader;
pub use self::shader::Shader;

mod texture;
pub use self::texture::Texture;

pub mod camera;
pub use camera::Camera;
