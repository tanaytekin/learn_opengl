use gl::types::*;
use glam::*;
use glfw::Context;
use std::ptr;

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;
const WINDOW_TITLE: &str = "learn_opengl";

mod graphics;
use graphics::*;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to init GLFW.");
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::Resizable(false));

    let (mut window, events) = glfw
        .with_primary_monitor(|glfw, m| {
            glfw.create_window(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                WINDOW_TITLE,
                m.map_or(glfw::WindowMode::Windowed, |m| {
                    glfw::WindowMode::FullScreen(m)
                }),
            )
        })
        .expect("Failed to create GLFW window.");

    window.set_cursor_mode(glfw::CursorMode::Disabled);

    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_framebuffer_size_polling(true);

    window.make_current();

    //    glfw.set_swap_interval(glfw::SwapInterval::Sync(0));

    gl::load_with(|s| window.get_proc_address(s) as *const _);
    let gl_version = graphics::gl_str_to_rust_string(gl::VERSION);
    println!("gl_version : {}", gl_version);

    let shader = Shader::new_from_paths("vert.glsl", "frag.glsl").expect("Shader compile error");

    let vertices: Vec<f32> = vec![
        -0.5, -0.5, -0.5, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, 0.5,
        -0.5, 1.0, 1.0, -0.5, 0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 0.0, -0.5, -0.5, 0.5,
        0.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 1.0, -0.5,
        0.5, 0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, 0.5, 0.5, 1.0, 0.0, -0.5, 0.5, -0.5,
        1.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0,
        0.0, -0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5,
        -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, 0.5, 0.0, 0.0, 0.5, 0.5, 0.5,
        1.0, 0.0, -0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, -0.5, 1.0, 1.0, 0.5, -0.5, 0.5, 1.0, 0.0,
        0.5, -0.5, 0.5, 1.0, 0.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, 0.5,
        -0.5, 0.0, 1.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0,
        -0.5, 0.5, 0.5, 0.0, 0.0, -0.5, 0.5, -0.5, 0.0, 1.0,
    ];

    let mut vao = 0;
    let mut vbo = 0;

    unsafe { gl::Enable(gl::DEPTH_TEST) };
    unsafe {
        gl::GenVertexArrays(1, &mut vao);

        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (5 * std::mem::size_of::<f32>()) as GLsizei,
            (0 * std::mem::size_of::<f32>()) as *const GLvoid,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            (5 * std::mem::size_of::<f32>()) as GLsizei,
            (3 * std::mem::size_of::<f32>()) as *const GLvoid,
        );
        gl::EnableVertexAttribArray(1);
    }

    let texture = Texture::from_path("container.jpg").expect("Texture create error");
    texture.bind();

    let mut model = Mat4::IDENTITY;
    let mut view = Mat4::from_translation(Vec3::new(0.0, 0.0, -3.0));
    let projection = Mat4::perspective_rh_gl(
        45.0_f32.to_radians(),
        (SCREEN_WIDTH as f32) / (SCREEN_HEIGHT as f32),
        0.1,
        100.0,
    );

    shader.bind();
    shader.set_mat4("model", &model);
    shader.set_mat4("projection", &projection);

    let mut camera = Camera::new();
    let mut first_mouse = false;
    let mut last_mouse_x = SCREEN_WIDTH as f32 / 2.0;
    let mut last_mouse_y = SCREEN_HEIGHT as f32 / 2.0;
    let mut last_time = glfw.get_time();

    while !window.should_close() {
        let cur_time = glfw.get_time();
        let delta_time = cur_time - last_time;
        last_time = cur_time;

        println!("FPS: {}", 1.0 / delta_time);

        if window.get_key(glfw::Key::W) == glfw::Action::Press {
            camera.process_keyboard(camera::Direction::FORWARD, delta_time as f32);
        } else if window.get_key(glfw::Key::S) == glfw::Action::Press {
            camera.process_keyboard(camera::Direction::BACKWARD, delta_time as f32);
        }

        if window.get_key(glfw::Key::A) == glfw::Action::Press {
            camera.process_keyboard(camera::Direction::LEFT, delta_time as f32);
        } else if window.get_key(glfw::Key::D) == glfw::Action::Press {
            camera.process_keyboard(camera::Direction::RIGHT, delta_time as f32);
        }

        camera.update();
        shader.set_mat4("view", &camera.view);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            shader.bind();
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true);
                }
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe { gl::Viewport(0, 0, width, height) };
                }
                glfw::WindowEvent::CursorPos(x, y) => {
                    let x = x as f32;
                    let y = y as f32;

                    if first_mouse {
                        last_mouse_x = x;
                        last_mouse_y = y;
                        first_mouse = false;
                    }

                    let x_offset = x - last_mouse_x;
                    let y_offset = last_mouse_y - y;

                    last_mouse_x = x;
                    last_mouse_y = y;

                    camera.process_mouse(x_offset, y_offset);
                }
                _ => {}
            }
        }
    }

    window.close();
}
