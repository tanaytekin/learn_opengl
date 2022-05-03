use gl::types::*;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::fs::read_to_string;
use std::ptr;

pub struct Shader {
    program: GLuint,
}

impl Shader {
    pub fn new(
        vertex_shader_src: &str,
        fragment_shader_src: &str,
    ) -> Result<Shader, Box<dyn Error>> {
        let vertex_shader_c_str = CString::new(vertex_shader_src)?;
        let vertex_shader = Shader::compile_shader(&vertex_shader_c_str, gl::VERTEX_SHADER)?;

        let fragment_shader_c_str = CString::new(fragment_shader_src)?;
        let fragment_shader = Shader::compile_shader(&fragment_shader_c_str, gl::FRAGMENT_SHADER)?;

        let program;

        unsafe {
            program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        Ok(Shader { program })
    }
    pub fn new_from_paths(
        vertex_shader_path: &str,
        fragment_shader_path: &str,
    ) -> Result<Shader, Box<dyn Error>> {
        let vertex_shader_src = read_to_string(vertex_shader_path)?;
        let fragment_shader_src = read_to_string(fragment_shader_path)?;

        Shader::new(&vertex_shader_src, &fragment_shader_src)
    }

    fn compile_shader(source: &CStr, shader_type: GLenum) -> Result<u32, Box<dyn Error>> {
        let shader;
        unsafe {
            if shader_type != gl::VERTEX_SHADER && shader_type != gl::FRAGMENT_SHADER {
                return Err(format!("Unsupported shader type {}", shader_type).into());
            }

            shader = gl::CreateShader(shader_type);
            gl::ShaderSource(shader, 1, &source.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            let mut success: GLint = 0;

            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                let mut info_log_len: GLint = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut info_log_len);

                let mut info_log: Vec<u8> = vec![0; info_log_len as usize];
                gl::GetShaderInfoLog(
                    shader,
                    info_log_len as GLsizei,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut i8,
                );
                let info_log_c_string = CString::from_vec_unchecked(info_log);
                return Err(info_log_c_string.to_str()?.into());
            }
        }
        Ok(shader)
    }

    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.program) };
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.program) };
    }
}
