use crate::camera::Camera;
use crate::camera::CameraMovement;
use crate::entity::entity::Entity;
use crate::light::Light;
use crate::log_builder;
use crate::material::Material;
use crate::material::UniformValue;
use crate::mesh::Vertex;
use crate::texture::{FilteringMode, WrappingMode};
use crate::transform::Transform;
use cgmath::Vector2;
use glfw::SwapInterval;
use glfw::ffi::glfwGetTime;
use glfw::{Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent};
use log::debug;
use log::error;
use log::info;
use log::warn;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct App {
    is_running: bool,
    window: Option<Rc<RefCell<PWindow>>>,
    glfw: Option<Rc<RefCell<Glfw>>>,
    event_receiver: Option<Rc<RefCell<GlfwReceiver<(f64, WindowEvent)>>>>,
    delta_time: f32,
    last_time: f32,
    last_cursor_pos: Vector2<f32>,
    is_first_cursor_move: bool,
    camera: Rc<RefCell<Camera>>,
    entities: Vec<Rc<RefCell<Entity>>>,
}

// main
impl App {
    pub fn new() -> Self {
        let app = Self {
            is_running: false,
            window: None,
            glfw: None,
            event_receiver: None,
            delta_time: 0.0,
            last_time: 0.0,
            last_cursor_pos: Vector2 { x: 0.0, y: 0.0 },
            is_first_cursor_move: true,
            camera: Rc::new(RefCell::new(Camera::new())),
            entities: Vec::new(),
        };

        log_builder::setup_logger();

        app
    }

    pub fn init_window(&mut self, width: u32, height: u32) {
        // 初始化 GLFW
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        // 设置窗口提示
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::Samples(Some(4)));

        // 创建窗口
        let (mut window, events) = glfw
            .create_window(
                width,
                height,
                "Rust GLFW opengl",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window");

        // 设置窗口为当前上下文
        window.make_current();
        // window.set_all_polling(true);
        window.set_key_polling(true); // 监听键盘输入
        window.set_scroll_polling(true); // 监听滚轮事件
        window.set_cursor_pos_polling(true); // 监听鼠标移动事件
        window.set_framebuffer_size_polling(true); // 监听窗口大小变化
        window.set_close_polling(true);

        // 开启垂直同步
        glfw.set_swap_interval(SwapInterval::Sync(1));
        // 不限制帧率
        // glfw.set_swap_interval(SwapInterval::None);

        // 加载 OpenGL 函数指针
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        // 显示版本
        unsafe {
            let version = gl::GetString(gl::VERSION);
            let version_str = std::ffi::CStr::from_ptr(version as *const _)
                .to_str()
                .unwrap_or("无法获取 OpenGL 版本");

            info!("OpenGL 版本: {}", version_str);
            let version = gl::GetString(gl::SHADING_LANGUAGE_VERSION);
            let version_str = std::ffi::CStr::from_ptr(version as *const _)
                .to_str()
                .unwrap_or("无法获取可支持的glsl 版本");
            info!("支持的 GLSL 版本: {:?}", version_str);
        }

        // 启用原始鼠标输入（避免系统加速影响）
        window.set_cursor_mode(glfw::CursorMode::Disabled);
        window.set_cursor_pos(width as f64 / 2.0, height as f64 / 2.0); // 初始居中

        // 初始化成员
        self.glfw = Some(Rc::new(RefCell::new(glfw)));
        self.window = Some(Rc::new(RefCell::new(window)));
        self.event_receiver = Some(Rc::new(RefCell::new(events)));

        // 抗锯齿
        unsafe {
            gl::Enable(gl::MULTISAMPLE);
        }

        // 初始化视口
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }
        self.camera.borrow_mut().set_aspect_ratio(width, height);

        // 窗口大小改变时，视口变化
        let camera_weak = Rc::downgrade(&self.camera);
        self.window
            .as_ref()
            .unwrap()
            .borrow_mut()
            .set_framebuffer_size_callback(move |_window, width, height| {
                debug!(
                    "window size change: width {:?}, height: {:?}",
                    width, height
                );
                unsafe {
                    gl::Viewport(0, 0, width, height);
                }
                camera_weak
                    .upgrade()
                    .unwrap()
                    .borrow_mut()
                    .set_aspect_ratio(width as u32, height as u32);
            });
    }

    pub fn add_entity(&mut self, entity: Rc<RefCell<Entity>>) {
        self.entities.push(entity);
    }

    pub fn set_camera_transform(&self, transform: Transform) {
        self.camera.borrow_mut().set_transform(transform);
    }

    pub fn run(&mut self) {
        self.is_running = true;

        info!("app starts to running...");

        while self.is_running {
            self.calc_delta_time();

            self.glfw.as_ref().unwrap().borrow_mut().poll_events();
            self.handle_window_event();

            self.render();

            self.window.as_ref().unwrap().borrow_mut().swap_buffers();
        }

        info!("app is going to close...");
    }
}

// utils
impl App {
    fn calc_delta_time(&mut self) {
        unsafe {
            let current_time = glfwGetTime() as f32;
            self.delta_time = (current_time - self.last_time).abs();
            self.last_time = current_time;
        }
        debug!("fps: {:?}", 1.0 / self.delta_time);
    }

    fn render(&mut self) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); // 每帧清除深度缓冲
        }
        let view_matrix = self.camera.borrow().get_view_matrix();
        let projection_matrix = self.camera.borrow().get_projection_matrix();
        let view_position = self
            .camera
            .borrow()
            .get_transform()
            .get_position()
            .get_arr();

        for (entity) in self.entities.iter() {
            let entity = entity.borrow();
            // 计算矩阵
            let model_matrix = entity.transform.to_matrix();
            let normal_matrix = entity.transform.to_normal_matrix();
            // 给材质注入全局变量，比如mvp
            entity
                .material
                .borrow_mut()
                .insert_uniform("view_position", UniformValue::Vector3(view_position));
            entity
                .material
                .borrow_mut()
                .insert_uniform("model_matrix", UniformValue::Matrix4(model_matrix));

            entity
                .material
                .borrow_mut()
                .insert_uniform("normal_matrix", UniformValue::Matrix3(normal_matrix));
            entity
                .material
                .borrow_mut()
                .insert_uniform("view_matrix", UniformValue::Matrix4(view_matrix));
            entity.material.borrow_mut().insert_uniform(
                "projection_matrix",
                UniformValue::Matrix4(projection_matrix),
            );
            // 通知opengl用这个材质，初始化
            entity.material.borrow().bind();
            // 通知opengl进行绘制
            entity.mesh.borrow().draw();
            // 通知opengl卸载这个材质
            entity.material.borrow().unbind();
        }
    }

    fn bind_material(&self, material: Rc<RefCell<Material>>) {
        let material = material.borrow();
        let shader = material.shader.borrow();

        shader.bind();
        // 给shader设置所有这个材质对应的uniforms
        for (name, value) in &material.uniforms {
            match value {
                UniformValue::Float(v) => shader.set_uniform_f32(name, *v),
                UniformValue::Int(v) => shader.set_uniform_i32(name, *v),
                UniformValue::Vector3(v) => shader.set_uniform_vec3(name, v),
                UniformValue::Vector4(v) => shader.set_uniform_vec4(name, v),
                UniformValue::Matrix3(m) => shader.set_uniform_mat3(name, m),
                UniformValue::Matrix4(m) => shader.set_uniform_mat4(name, m),
                UniformValue::Texture(slot) => shader.set_uniform_i32(name, *slot as i32),
            }
        }

        for (texture_slot_id, texture) in &material.textures {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + texture_slot_id);
                gl::BindTexture(gl::TEXTURE_2D, texture.borrow().get_id());
            }
        }
    }

    fn unbind_material(&self, material: Rc<RefCell<Material>>) {
        material.borrow().shader.borrow().unbind();
    }

    fn handle_window_event(&mut self) {
        for (_, event) in
            glfw::flush_messages(&mut *(self.event_receiver.as_ref().unwrap().borrow_mut()))
        {
            match event {
                WindowEvent::Key(key, _, action, _) => {
                    debug!("Trigger Key {:?} {:?}", key, action);
                    if key == Key::Escape {
                        debug!("Trigger WindowClose");
                        self.window
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .set_should_close(true);
                        self.is_running = false;
                    }

                    let velocity = 40.0;
                    let movement = match key {
                        Key::W => Some(CameraMovement::Forward),
                        Key::A => Some(CameraMovement::Left),
                        Key::S => Some(CameraMovement::Backward),
                        Key::D => Some(CameraMovement::Right),
                        Key::LeftShift => Some(CameraMovement::Down),
                        Key::Space => Some(CameraMovement::Up),
                        _ => None,
                    };
                    if let Some(movement) = movement {
                        self.camera
                            .borrow_mut()
                            .process_move(movement, velocity, self.delta_time);
                    }
                }
                WindowEvent::Close => {
                    debug!("Trigger WindowClose");
                    self.window
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        .set_should_close(true);
                    self.is_running = false;
                }
                WindowEvent::Scroll(xoffset, yoffset) => {
                    debug!("Trigger Mouse Scroll: X={}, Y={}", xoffset, yoffset);
                    self.camera
                        .clone()
                        .borrow_mut()
                        .process_zoom(yoffset as f32, 0.5);
                }
                WindowEvent::CursorPos(xpos, ypos) => {
                    debug!("Trigger Cursor Move: X={}, Y={}", xpos, ypos);
                    if self.is_first_cursor_move {
                        self.is_first_cursor_move = false;
                        self.last_cursor_pos = Vector2::new(xpos as f32, ypos as f32);
                    } else {
                        let xoffset = xpos as f32 - self.last_cursor_pos.x;
                        let yoffset = ypos as f32 - self.last_cursor_pos.y;
                        self.camera
                            .clone()
                            .borrow_mut()
                            .process_turn(xoffset, yoffset, 0.001, true);
                        self.last_cursor_pos = Vector2::new(xpos as f32, ypos as f32);
                    }
                }
                WindowEvent::MouseButton(button, action, _) => {
                    debug!("Trigger Mouse button: {:?}, Action: {:?}", button, action);
                }
                _ => (),
            };
        }
    }
}
