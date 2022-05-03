use gl::types::*;
use glfw::Context;
use std::ptr;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const WINDOW_TITLE: &str = "learn_opengl";

mod graphics;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to init GLFW.");
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::Resizable(false));

    let (mut window, events) = glfw
        .create_window(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            WINDOW_TITLE,
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();
    glfw.set_swap_interval(glfw::SwapInterval::Sync(0));

    gl::load_with(|s| window.get_proc_address(s) as *const _);
    let gl_version = graphics::gl_str_to_rust_string(gl::VERSION);
    println!("gl_version : {}", gl_version);

    let shader =
        graphics::Shader::new_from_paths("vert.glsl", "frag.glsl").expect("Shader compile error");

    let vertices: Vec<f32> = vec![
        // positions      // colors        // texture coords
        0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top right
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom right
        -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom left
        -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, // top left
    ];

    let indices: Vec<u32> = vec![
        0, 1, 3, // first triangle
        1, 2, 3, // second triangle
    ];

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
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as GLsizeiptr,
            indices.as_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * std::mem::size_of::<f32>()) as GLsizei,
            (0 * std::mem::size_of::<f32>()) as *const GLvoid,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * std::mem::size_of::<f32>()) as GLsizei,
            (3 * std::mem::size_of::<f32>()) as *const GLvoid,
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            (8 * std::mem::size_of::<f32>()) as GLsizei,
            (6 * std::mem::size_of::<f32>()) as *const GLvoid,
        );
        gl::EnableVertexAttribArray(2);
    }

    let texture = graphics::Texture::from_path("container.jpg").expect("Texture create error");
    texture.bind();

    let mut last_time = glfw.get_time();

    while !window.should_close() {
        let cur_time = glfw.get_time();
        let delta_time = cur_time - last_time;
        last_time = cur_time;

        println!("FPS: {}", 1.0 / delta_time);

        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            shader.bind();
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true);
                }
                _ => {}
            }
        }
    }

    window.close();
}
