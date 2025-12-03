use crate::render::camera::Camera;
use crate::render::camera::CameraMovement;
use crate::core::FixedUpdateAble;
use crate::render::entity::entity::Entity;
use crate::event::event::AppEvent;
use crate::event::event_queue;
use crate::event::event_queue::AppEventQueue;
use crate::input::input_state;
use crate::input::input_state::InputState;
use crate::render::light::Light;
use crate::log_builder;
use crate::render::material::Material;
use crate::render::material::UniformValue;
use crate::render::mesh::Vertex;
use crate::render::texture::{FilteringMode, WrappingMode};
use crate::render::transform::Transform;
use crate::render::world::world::World;
use cgmath::Vector2;
use glfw::Action;
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
use std::time::Duration;
use std::time::Instant;

pub struct AppConfig {
    pub title: String,
    pub target_render_fps: Option<u32>, // None = Unlimited
    pub fixed_update_fps: u32,          // e.g. 60
    pub v_sync: bool,
    pub anti_pixel_msaa: Option<u32>,   // e.g. 4
}

pub struct App {
    config: AppConfig,

    is_running: bool,
    window: Option<Rc<RefCell<PWindow>>>,
    glfw: Option<Rc<RefCell<Glfw>>>,
    event_receiver: Option<Rc<RefCell<GlfwReceiver<(f64, WindowEvent)>>>>,

    world: Rc<RefCell<World>>,
    input_state: Rc<RefCell<InputState>>,
    event_queue: Rc<RefCell<AppEventQueue>>,
}

// main
impl App {
    pub fn new() -> Rc<RefCell<Self>> {
        Self::new_with_config(AppConfig {
            title: String::from("Rust GLFW opengl"),
            target_render_fps: Some(60),
            fixed_update_fps: 30,
            v_sync: true,
            anti_pixel_msaa: Some(4),
        })
    }

    pub fn new_with_config(config: AppConfig) -> Rc<RefCell<Self>> {
        let app = Self {
            config: config,
            is_running: false,
            window: None,
            glfw: None,
            event_receiver: None,
            world: Rc::new(RefCell::new(World::new())),
            input_state: Rc::new(RefCell::new(InputState::new())),
            event_queue: Rc::new(RefCell::new(AppEventQueue::new())),
        };

        log_builder::setup_logger();

        Rc::new(RefCell::new(app))
    }

    pub fn init_window(&mut self, width: u32, height: u32) {
        // 初始化 GLFW
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        // 设置窗口提示
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::Samples(self.config.anti_pixel_msaa));

        // 创建窗口
        let (mut window, events) = glfw
            .create_window(
                width,
                height,
                self.config.title.as_str(),
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
        if self.config.v_sync {
            glfw.set_swap_interval(SwapInterval::Sync(1));
        } else {
            glfw.set_swap_interval(SwapInterval::None);
        }

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
        if self.config.anti_pixel_msaa.is_some() {
            unsafe {
                gl::Enable(gl::MULTISAMPLE);
            }
        }

        // 初始化视口
        self.resize_view(width, height);
    }

    pub fn get_world(&self) -> Rc<RefCell<World>> {
        self.world.clone()
    }

    pub fn run(&mut self) {
        self.is_running = true;

        info!("app starts to running...");

        let fixed_dt = 1.0 / self.config.fixed_update_fps as f32;
        let target_render_dt = self.config.target_render_fps.map(|fps| 1.0 / fps as f32);

        let mut last_time = self.get_current_time();
        let mut last_render_update_time = last_time;
        let mut last_fixed_update_time = last_time;

        while self.is_running {
            // 计算事件
            let now = self.get_current_time();
            let delta_time = now - last_time;
            last_time = now;

            // glfw事件
            self.glfw.as_ref().unwrap().borrow_mut().poll_events();
            self.handle_window_event();

            //处理事件队列
            self.handle_event_queue();

            // FixedUpdate 循环
            while now - last_fixed_update_time >= fixed_dt {
                self.fixed_update(fixed_dt);
                last_fixed_update_time += fixed_dt
            }

            // Render 限帧
            if let Some(render_dt) = target_render_dt {
                let since_last_render = now - last_render_update_time;
                // 比限制的render间隔少的时间就跑完了一次render，那就休息一会把
                if since_last_render < render_dt {
                    // sleep 少量时间减少 CPU 占用
                    std::thread::sleep(Duration::from_millis(1));
                    continue;
                }
            }

            // 渲染
            last_render_update_time = self.get_current_time();
            self.render_update();
            self.window.as_ref().unwrap().borrow_mut().swap_buffers();
        }

        info!("app is going to close...");
    }
}

// utils
impl App {
    fn handle_event_queue(&mut self) {
        let events = self.event_queue.borrow_mut().drain();
        for event in events {
            match event {
                AppEvent::Resize { width, height } => self.resize_view(width as u32, height as u32),
                AppEvent::Key { key, action } => match action {
                    Action::Press => {
                        self.input_state.borrow_mut().press_key(key);
                    }
                    Action::Release => {
                        self.input_state.borrow_mut().release_key(&key);
                    }
                    _ => {}
                },
                AppEvent::Close => {
                    self.close();
                }
                AppEvent::Scroll { x, y } => {
                    self.input_state.borrow_mut().set_scroll_delta(x, y);
                }
                AppEvent::CursorPos { x, y } => {
                    self.input_state.borrow_mut().set_cursor_delta(x, y);
                }
                AppEvent::MouseButton { button, action } => match action {
                    Action::Press => {
                        self.input_state.borrow_mut().press_mouse_button(button);
                    }
                    Action::Release => {
                        self.input_state.borrow_mut().release_mouse_button(&button);
                    }
                    _ => {}
                },
            }
        }
    }

    fn resize_view(&self, width: u32, height: u32) {
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }
        self.get_world()
            .borrow()
            .get_camera()
            .borrow_mut()
            .set_aspect_ratio(width, height);
    }

    fn fixed_update(&mut self, fixed_dt: f32) {
        if self.input_state.borrow().is_key_down(Key::Escape) {
            self.window
                .as_ref()
                .unwrap()
                .borrow_mut()
                .set_should_close(true);
            self.is_running = false;
        }

        self.world
            .borrow()
            .get_camera()
            .borrow_mut()
            .fixed_update(fixed_dt, self.input_state.clone());

        self.input_state.borrow_mut().clear_delta();
    }

    fn get_current_time(&self) -> f32 {
        unsafe {
            let current_time = glfwGetTime() as f32;
            current_time
        }
    }

    fn render_update(&mut self) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); // 每帧清除深度缓冲
        }
        let camera = self.get_world().borrow().get_camera();
        let view_matrix = camera.borrow().get_view_matrix();
        let projection_matrix = camera.borrow().get_projection_matrix();
        let view_position = camera.borrow().get_view_position();
        // let light_color = self.light.get_color();
        // let light_position = self.light.get_transform().get_position().get_arr();

        for (entity) in self.get_world().borrow().get_entities().iter() {
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
            // entity
            //     .material
            //     .borrow_mut()
            //     .insert_uniform("light_color", UniformValue::Vector4(light_color));
            // entity
            //     .material
            //     .borrow_mut()
            //     .insert_uniform("light_position", UniformValue::Vector3(light_position));

            // 通知opengl用这个材质，初始化
            entity.material.borrow().bind();
            // 通知opengl进行绘制
            entity.mesh.borrow().draw();
            // 通知opengl卸载这个材质
            entity.material.borrow().unbind();
        }
    }

    fn close(&mut self) {
        self.window
            .as_ref()
            .unwrap()
            .borrow_mut()
            .set_should_close(true);
        self.is_running = false;
    }

    fn handle_window_event(&mut self) {
        let input = self.input_state.clone();
        for (_, event) in
            glfw::flush_messages(&mut *(self.event_receiver.as_ref().unwrap().borrow_mut()))
        {
            match event {
                WindowEvent::Key(key, _, action, _) => {
                    self.event_queue
                        .borrow_mut()
                        .push(AppEvent::Key { key, action });
                }
                WindowEvent::Close => {
                    self.event_queue.borrow_mut().push(AppEvent::Close);
                }
                WindowEvent::Scroll(xoffset, yoffset) => {
                    self.event_queue.borrow_mut().push(AppEvent::Scroll {
                        x: xoffset,
                        y: yoffset,
                    });
                }
                WindowEvent::CursorPos(xpos, ypos) => {
                    self.event_queue
                        .borrow_mut()
                        .push(AppEvent::CursorPos { x: xpos, y: ypos });
                }
                WindowEvent::MouseButton(button, action, _) => {
                    self.event_queue
                        .borrow_mut()
                        .push(AppEvent::MouseButton { button, action });
                }
                WindowEvent::FramebufferSize(width, height) => {
                    self.event_queue
                        .borrow_mut()
                        .push(AppEvent::Resize { width, height });
                }
                _ => (),
            };
        }
    }
}
