use super::texture::Texture;
use super::texture::TextureType;

use super::shader::Shader;
use gl::types::*;
use glam::*;

#[repr(packed)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub tex_coords: Vec2,
}

#[derive(Default)]
pub struct Mesh<'a> {
    vertices: Vec<Vertex>,
    indices: Vec<GLuint>,
    textures: Vec<&'a Texture>,
    vao: GLuint,
}

impl<'a> Mesh<'a> {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<GLuint>, textures: Vec<&'a Texture>) -> Mesh {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);

            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<Vertex>()) as GLsizeiptr,
                vertices.as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
                vertices.as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            );

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (std::mem::size_of::<Vertex>()) as GLsizei,
                (0 * std::mem::size_of::<f32>()) as *const GLvoid,
            );

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (std::mem::size_of::<Vertex>()) as GLsizei,
                (3 * std::mem::size_of::<f32>()) as *const GLvoid,
            );

            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                (std::mem::size_of::<Vertex>()) as GLsizei,
                (6 * std::mem::size_of::<f32>()) as *const GLvoid,
            );
            
            gl::BindVertexArray(0);
        }

        Mesh {
            vertices,
            indices,
            textures,
            vao,
        }
    }

    pub fn draw(&self, shader: &Shader) {
        let mut diffuse_n = 1;
        let mut specular_n = 1;
        
        shader.use_shader();
        for (i, texture) in self.textures.iter().enumerate() {
            texture.bind(i as u32);

            let name;
            match texture.texture_type {
                TextureType::DIFFUSE => {
                    name = format!("material.diffuse{}", diffuse_n);
                    diffuse_n += 1;
                },
                TextureType::SPECULAR => {
                    name = format!("material.specular{}", specular_n);
                    specular_n += 1;
                },
                _ => todo!("Unimplemented yet."),
            }

            shader.set_i32(&name, i as i32);
        }


        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as GLsizei, gl::UNSIGNED_INT, 0 as *const GLvoid);

        }


    }
}
