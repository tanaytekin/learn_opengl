use gl::types::*;
use glam::*;
use glfw::Context;

use egui_backend::egui::{vec2, Pos2, Rect};
use egui_glfw_gl as egui_backend;
use std::time::Instant;

#[cfg(feature = "fullscreen")]
const SCREEN_WIDTH: u32 = 1920;
#[cfg(feature = "fullscreen")]
const SCREEN_HEIGHT: u32 = 1080;

#[cfg(not(feature = "fullscreen"))]
const SCREEN_WIDTH: u32 = 1280;
#[cfg(not(feature = "fullscreen"))]
const SCREEN_HEIGHT: u32 = 720;

const WINDOW_TITLE: &str = "learn_opengl";

mod graphics;
use graphics::*;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to init GLFW.");
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));
    glfw.window_hint(glfw::WindowHint::Resizable(false));

    let (mut window, events) = glfw
        .with_primary_monitor(|glfw, m| {
            glfw.create_window(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                WINDOW_TITLE,
                #[cfg(feature = "fullscreen")]
                m.map_or(glfw::WindowMode::Windowed, |m| {
                    glfw::WindowMode::FullScreen(m)
                }),
                #[cfg(not(feature = "fullscreen"))]
                m.map_or(glfw::WindowMode::Windowed, |_m| glfw::WindowMode::Windowed),
            )
        })
        .expect("Failed to create GLFW window.");

    window.set_cursor_mode(glfw::CursorMode::Disabled);
    let mut cursor_disabled = true;

    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_framebuffer_size_polling(true);

    window.set_char_polling(true);
    window.set_mouse_button_polling(true);
    window.make_current();

    glfw.set_swap_interval(glfw::SwapInterval::Sync(0));

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let mut painter = egui_backend::Painter::new(&mut window, SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut egui_ctx = egui::CtxRef::default();

    let (width, height) = window.get_framebuffer_size();
    let native_pixels_per_point = window.get_content_scale().0;

    let mut egui_input_state = egui_backend::EguiInputState::new(egui::RawInput {
        screen_rect: Some(Rect::from_min_size(
            Pos2::new(0f32, 0f32),
            vec2(width as f32, height as f32) / native_pixels_per_point,
        )),
        pixels_per_point: Some(native_pixels_per_point),
        ..Default::default()
    });

    let gl_version = graphics::gl_str_to_rust_string(gl::VERSION);
    println!("gl_version : {}", gl_version);

    let lighting_shader = Shader::from_paths("lighting_vert.glsl", "lighting_frag.glsl")
        .expect("Shader compile error");

    let light_cube_shader = Shader::from_paths("light_cube_vert.glsl", "light_cube_frag.glsl")
        .expect("Shader compile error");


    let vertices: Vec<f32> = vec![
               // positions          // normals           // texture coords
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
        -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  1.0,
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,

        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,
         0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
        -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  1.0,
        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,

        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,
        -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0,  1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
        -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0,  0.0,
        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,

         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0,  1.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
         0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0,  0.0,
         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,

        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,
         0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0,  1.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
        -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0,  0.0,
        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,

        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0,
         0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
        -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0,  0.0,
        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0
    ];

    //let mut cube_vao = 0;
    let mut light_cube_vao = 0;
    let mut vbo = 0;

    unsafe { gl::Enable(gl::DEPTH_TEST) };
    unsafe {
        //gl::GenVertexArrays(1, &mut cube_vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW,
        );
/*
        gl::BindVertexArray(cube_vao);

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
*/
        gl::GenVertexArrays(1, &mut light_cube_vao);
        gl::BindVertexArray(light_cube_vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * std::mem::size_of::<f32>()) as GLsizei,
            (0 * std::mem::size_of::<f32>()) as *const GLvoid,
        );
        gl::EnableVertexAttribArray(0);
    }
/*
    let diffuse_map = Texture::from_path("container2.png").unwrap();
    let specular_map = Texture::from_path("container2_specular.png").unwrap();
    lighting_shader.use_shader();
    lighting_shader.set_i32("material.diffuse_tex", 0);
    lighting_shader.set_i32("material.specular_tex", 1);
 */

    let mut camera = Camera::new();
    let mut first_mouse = false;
    let mut last_mouse_x = SCREEN_WIDTH as f32 / 2.0;
    let mut last_mouse_y = SCREEN_HEIGHT as f32 / 2.0;

    let mut light_pos = Vec3::new(1.2, 1.0, 2.0);
    let mut cube_pos = Vec3::new(0.0, 0.0, 0.0);

    let mut last_time = glfw.get_time();

    let start_time = Instant::now();
    let mut quit = false;

    let mut light_ambient = Vec3::new(0.1, 0.1, 0.1);
    let mut light_diffuse = Vec3::new(0.5, 0.5, 0.5);
    let mut light_specular = Vec3::new(1.0, 1.0, 1.0);

    let mut material_ambient = Vec3::new(1.0, 1.0, 1.00);
    let mut material_diffuse = Vec3::new(1.0, 1.0, 1.00);
    let mut material_specular = Vec3::new(1.0, 1.0, 1.0);

    let mut material_shininess = 32.0;



    let mut md = Model::new("backpack.obj").unwrap();



    while !window.should_close() {
        egui_input_state.input.time = Some(start_time.elapsed().as_secs_f64());
        egui_ctx.begin_frame(egui_input_state.input.take());

        egui_input_state.input.pixels_per_point = Some(native_pixels_per_point);

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

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        lighting_shader.use_shader();
        lighting_shader.set_vec3v("view_pos", &camera.position);

        lighting_shader.set_vec3v("light.position", &light_pos);
        lighting_shader.set_vec3v("light.ambient", &light_ambient);
        lighting_shader.set_vec3v("light.diffuse", &light_diffuse);
        lighting_shader.set_vec3v("light.specular", &light_specular);


        let projection = Mat4::perspective_rh_gl(
            45.0_f32.to_radians(),
            (SCREEN_WIDTH as f32) / (SCREEN_HEIGHT as f32),
            0.1,
            100.0,
        );

        let view = camera.view;
        let model = Mat4::from_translation(cube_pos);

    /*
        diffuse_map.bind(0);
        specular_map.bind(1);
        */

        lighting_shader.set_mat4v("projection", &projection);
        lighting_shader.set_mat4v("view", &view);
        lighting_shader.set_mat4v("model", &model);
/*
        unsafe {
            gl::BindVertexArray(cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
 */

        light_cube_shader.use_shader();
        light_cube_shader.set_mat4v("projection", &projection);
        light_cube_shader.set_mat4v("view", &view);
        let mut light_color = (light_ambient + light_diffuse + light_specular)/3.0;
        light_color.normalize();
        light_cube_shader.set_vec3v("light_color", &light_color);



        //let mut model = Mat4::from_translation(Vec3::new(1.2, 1.0, 2.0));
        let mut model = Mat4::from_translation(light_pos);
        model = model * Mat4::from_scale(Vec3::new(1.0, 1.0, 1.0));
        light_cube_shader.set_mat4v("model", &model);

        unsafe {
            gl::BindVertexArray(light_cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
        /*
        model_shader.use_shader();
        model_shader.set_mat4v("projection", &projection);
        model_shader.set_mat4v("view", &view);
        model_shader.set_mat4v("model", &model);
         */

        md.draw(&lighting_shader);

        /*
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            shader.bind();
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
        */

        egui::Window::new("Controls").show(&egui_ctx, |ui| {
            ui.heading("Model Position");
            ui.add(egui::Slider::new(&mut cube_pos.x, -10.0..=10.0).text("x"));
            ui.add(egui::Slider::new(&mut cube_pos.y, -10.0..=10.0).text("y"));
            ui.add(egui::Slider::new(&mut cube_pos.z, -10.0..=10.0).text("z"));

            ui.heading("Light Position");
            ui.add(egui::Slider::new(&mut light_pos.x, -10.0..=10.0).text("x"));
            ui.add(egui::Slider::new(&mut light_pos.y, -10.0..=10.0).text("y"));
            ui.add(egui::Slider::new(&mut light_pos.z, -10.0..=10.0).text("z"));
/*
            ui.heading("Material Color");
            ui.horizontal(|ui| {
                ui.label("Ambient");
                ui.color_edit_button_rgb(&mut material_ambient.as_mut());
            });
            ui.horizontal(|ui| {
                ui.label("Diffuse");
                ui.color_edit_button_rgb(&mut material_diffuse.as_mut());
            });
            ui.horizontal(|ui| {
                ui.label("Specular");
                ui.color_edit_button_rgb(&mut material_specular.as_mut());
            });
            ui.horizontal(|ui| {
                ui.label("Shininess");
                ui.add(egui::Slider::new(&mut material_shininess, 0.0..=128.0));
            });
            */

            ui.heading("Light Color");
            ui.horizontal(|ui| {
                ui.label("Ambient");
                ui.color_edit_button_rgb(&mut light_ambient.as_mut());
            });
            ui.horizontal(|ui| {
                ui.label("Diffuse");
                ui.color_edit_button_rgb(&mut light_diffuse.as_mut());
            });
            ui.horizontal(|ui| {
                ui.label("Specular");
                ui.color_edit_button_rgb(&mut light_specular.as_mut());
            });

            if ui.button("Quit").clicked() {
                quit = true;
            }
        });

        //light_pos.x = light_pos_x;

        let (_, paint_cmds) = egui_ctx.end_frame();

        let paint_jobs = egui_ctx.tessellate(paint_cmds);

        //Note: passing a bg_color to paint_jobs will clear any previously drawn stuff.
        //Use this only if egui is being used for all drawing and you aren't mixing your own Open GL
        //drawing calls with it.
        //Since we are custom drawing an OpenGL Triangle we don't need egui to clear the background.

        unsafe { gl::Disable(gl::DEPTH_TEST) };
        painter.paint_jobs(
            None,
            paint_jobs,
            &egui_ctx.texture(),
            native_pixels_per_point,
        );
        unsafe { gl::Enable(gl::DEPTH_TEST) };

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
                    egui_backend::handle_event(event, &mut egui_input_state);
                }
                glfw::WindowEvent::MouseButton(mouse_button, action, _) => {
                    egui_backend::handle_event(event, &mut egui_input_state);
                    if mouse_button == glfw::MouseButton::Button2 && action == glfw::Action::Press {
                        cursor_disabled = !cursor_disabled;
                        if cursor_disabled {
                            window.set_cursor_mode(glfw::CursorMode::Disabled);
                            camera.lock(false);
                        } else {
                            window.set_cursor_mode(glfw::CursorMode::Normal);
                            camera.lock(true);
                        }
                    }
                }
                _ => {
                    egui_backend::handle_event(event, &mut egui_input_state);
                }
            }
        }

        if quit {
            break;
        }
    }

    window.close();
}
