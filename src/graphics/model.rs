use super::texture::Texture;
use super::shader::Shader;

use gl::types::*;
use glam::*;

use std::error::Error;

use std::rc::Rc;


#[repr(packed)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub tex_coords: Vec2,
}

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<GLuint>,
    material: Rc<Material>,
    vao: GLuint,
}


pub struct Material {
    pub name: String,
    pub ambient_color: Vec3,
    pub diffuse_color: Vec3,
    pub specular_color: Vec3,
    pub diffuse_texture: Option<Texture>,
    pub specular_texture: Option<Texture>,
    pub shininess: f32,
}

#[derive(Default)]
pub struct Model {
    materials: Vec<Rc<Material>>,
    meshes: Vec<Mesh>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<GLuint>, material: Rc<Material>) -> Mesh{
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
                indices.as_ptr() as *const std::ffi::c_void,
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
            material,
            vao,
        }
    }

    pub fn draw(&self, shader: &Shader) {
        shader.use_shader();

        if let Some(difuse_texture) = &self.material.diffuse_texture {
            shader.set_i32("material.diffuse_tex", 0 as i32);
            difuse_texture.bind(0);
        }

        if let Some(specular_texture) = &self.material.specular_texture {
            shader.set_i32("material.specular_tex", 1 as i32);
            specular_texture.bind(1);
        }

        shader.set_vec3v("material.ambient", &self.material.ambient_color);
        shader.set_vec3v("material.diffuse", &self.material.diffuse_color);
        shader.set_vec3v("material.specular", &self.material.specular_color);
        shader.set_f32("material.shininess", self.material.shininess);

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES,
                             self.indices.len() as GLsizei,
                             gl::UNSIGNED_INT,
                             0 as *const GLvoid);

        }
    }
}

impl Model {
    pub fn new(path: &str) -> Result<Model, Box<dyn Error>> {
        let mut materials = Vec::new();
        let mut meshes = Vec::new();


        let mut tobj_load_options = tobj::LoadOptions::default();
        tobj_load_options.single_index = true;
        tobj_load_options.triangulate = true;


        let (models, tobj_materials) =
            tobj::load_obj(path, &tobj_load_options)?;

        let tobj_materials = tobj_materials?;
        
        for tobj_material in &tobj_materials {
            let mut diffuse_texture = None;
            let mut specular_texture = None;

            if !tobj_material.diffuse_texture.is_empty() {
                diffuse_texture = Some(Texture::from_path(tobj_material.diffuse_texture.as_str())?);
            }
            
            if !tobj_material.specular_texture.is_empty() {
                specular_texture = Some(Texture::from_path(tobj_material.specular_texture.as_str())?);
            }



            let ambient_color = Vec3::from_slice(&tobj_material.ambient);
            let diffuse_color = Vec3::from_slice(&tobj_material.diffuse);
            let specular_color = Vec3::from_slice(&tobj_material.specular);
            let shininess =  tobj_material.shininess;


            let material = Rc::new(Material {
                name: tobj_material.name.clone(),
                ambient_color,
                diffuse_color,
                specular_color,
                diffuse_texture,
                specular_texture,
                shininess,
            });

            materials.push(material);
        }

        for model in models {
            let tobj_mesh = &model.mesh;
            
        
            let mut vertices = Vec::new();
            let mut indices = Vec::new();
            

            let vertices_count = tobj_mesh.positions.len()/3;
            let normals_count = tobj_mesh.normals.len()/3;
            let texcoords_count = tobj_mesh.texcoords.len()/2;

            assert_eq!(vertices_count, normals_count);
            assert_eq!(normals_count, texcoords_count);


            for i in 0..vertices_count {
                let position = Vec3::new(
                    tobj_mesh.positions[i*3 + 0],
                    tobj_mesh.positions[i*3 + 1],
                    tobj_mesh.positions[i*3 + 2]
                );

                let normal = Vec3::new(
                    tobj_mesh.normals[i*3 + 0],
                    tobj_mesh.normals[i*3 + 1],
                    tobj_mesh.normals[i*3 + 2]
                );

                let tex_coords = Vec2::new(
                    tobj_mesh.texcoords[i*2 + 0],
                    tobj_mesh.texcoords[i*2 + 1],
                    );

                let vertex = Vertex{
                    position,
                    normal,
                    tex_coords,
                };

                vertices.push(vertex);
            }

            for idx in &tobj_mesh.indices {
                indices.push(idx.clone() as GLuint);
            }


            let tobj_material = &tobj_materials[tobj_mesh.material_id.unwrap()];

            let mut material_found = false;
            let mut mesh_material= materials[0].clone();

            for material in &materials {
                if tobj_material.name.eq(&material.name) {
                    mesh_material = material.clone();
                    material_found = true;
                    break;
                }
            }

            if !material_found {
                return Err("Cannot find material required for mesh".into());
            }

            
            let mesh = Mesh::new(vertices, indices, mesh_material);
            meshes.push(mesh);

        }

        Ok(Model{
            materials,
            meshes
        })
    }



    pub fn draw(&self, shader: &Shader) {
        for mesh in &self.meshes {
            mesh.draw(shader);
        }
    }
}
