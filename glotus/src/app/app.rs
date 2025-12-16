use crate::{
    AppConfig,
    event::*,
    input::*,
    render::*,
    tick::*,
    utils::{self},
};
use glfw::{
    Action, Context, Glfw, GlfwReceiver, Key, PWindow, SwapInterval, WindowEvent, ffi::glfwGetTime,
};
use log::{error, info};
use std::{cell::RefCell, rc::Rc, time::Duration};

pub struct App {
    config: AppConfig,

    is_running: bool,
    window: Option<Rc<RefCell<PWindow>>>,
    glfw: Option<Rc<RefCell<Glfw>>>,
    event_receiver: Option<Rc<RefCell<GlfwReceiver<(f64, WindowEvent)>>>>,

    world: Rc<RefCell<World>>,
    input_state: Rc<RefCell<InputState>>,
    event_queue: Rc<RefCell<AppEventQueue>>,
    ticker: Rc<RefCell<Ticker>>,

    pipeline: Rc<RefCell<Pipeline>>,
}

// main
impl App {
    pub fn new() -> Rc<RefCell<Self>> {
        Self::new_with_config(Default::default())
    }

    pub fn new_with_config(config: AppConfig) -> Rc<RefCell<Self>> {
        let pipeline = Rc::new(RefCell::new((*config.pipeline_builder)()));
        let app = Self {
            config,
            is_running: false,
            window: None,
            glfw: None,
            event_receiver: None,
            world: Rc::new(RefCell::new(World::new())),
            input_state: Rc::new(RefCell::new(InputState::new())),
            event_queue: Rc::new(RefCell::new(AppEventQueue::new())),
            ticker: Rc::new(RefCell::new(Ticker::new())),
            pipeline,
        };

        utils::setup_logger();

        let app_rc = Rc::new(RefCell::new(app));

        app_rc.borrow_mut().init_window();

        app_rc
    }

    fn init_window(&mut self) {
        let width = self.config.width;
        let height = self.config.height;
        // 初始化 GLFW
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        // 设置窗口提示
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::Samples(
            self.config.anti_pixel_msaa.to_num(),
        ));

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
        if self.config.anti_pixel_msaa.to_num().is_some() {
            unsafe {
                gl::Enable(gl::MULTISAMPLE);
            }
        }

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        // 初始化视口
        self.resize_view(width, height);
    }

    pub fn get_world(&self) -> Rc<RefCell<World>> {
        self.world.clone()
    }

    pub fn get_ticker(&self) -> Rc<RefCell<Ticker>> {
        self.ticker.clone()
    }

    pub fn init_camera_tickable(&mut self) {
        self.ticker
            .borrow_mut()
            .add_tickable(Rc::new(RefCell::new(CameraTickable::new(
                self.world.borrow().get_camera().clone(),
            ))));
    }

    pub fn run(&mut self) {
        self.is_running = true;

        info!("app starts to running...");

        let fixed_dt = 1.0 / self.config.fixed_update_fps as f32;
        let target_render_dt = self.config.target_render_fps.map(|fps| 1.0 / fps as f32);

        let last_time = self.get_current_time();
        let mut last_render_update_time = last_time;
        let mut last_fixed_update_time = last_time;

        // FPS 统计
        let mut frame_count = 0;
        let mut fps_timer = last_time;

        while self.is_running {
            // 计算事件
            let now = self.get_current_time();

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
                let remaining = render_dt - since_last_render;

                if remaining > 0.0 {
                    // 如果剩余时间 > 4ms，睡眠 50% 的时间
                    if remaining > 0.005 {
                        std::thread::sleep(Duration::from_secs_f32(remaining * 0.9));
                    }
                    // 最后自旋等待，确保精确时间
                    loop {
                        let now = self.get_current_time();
                        if now - last_render_update_time >= render_dt {
                            break;
                        }

                        // 让出 CPU 时间片，但不睡眠
                        std::thread::yield_now();
                    }
                }
            }

            // 渲染
            self.render_update();
            self.window.as_ref().unwrap().borrow_mut().swap_buffers();

            last_render_update_time = self.get_current_time();

            // FPS 统计
            frame_count += 1;
            if last_render_update_time - fps_timer >= 1.0 {
                let actual_fps = frame_count as f32 / (last_render_update_time - fps_timer);
                info!(
                    "FPS: {:.1} | Frame Time: {:.3}ms",
                    actual_fps,
                    1000.0 / actual_fps
                );
                frame_count = 0;
                fps_timer = last_render_update_time;
            }
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

        self.ticker
            .borrow_mut()
            .tick_all(fixed_dt, self.input_state.clone());

        self.input_state.borrow_mut().clear_delta();
    }

    fn get_current_time(&self) -> f32 {
        unsafe {
            let current_time = glfwGetTime() as f32;
            current_time
        }
    }

    fn render_update(&mut self) {
        // 清空
        unsafe {
            gl::ClearColor(
                self.config.bg_color[0],
                self.config.bg_color[1],
                self.config.bg_color[2],
                1.0,
            );
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }

        // 计算全局数据
        let camera = self.get_world().borrow().get_camera();
        let view_matrix = camera.borrow().get_view_matrix();
        let projection_matrix = camera.borrow().get_projection_matrix();
        let view_position = camera.borrow().get_view_position();

        let lights_shader_data: Vec<LightShaderData> =
            self.get_world().borrow().get_light_shader_data();
        let light_count = lights_shader_data.len() as i32;
        let camera_shader_data = self.get_world().borrow().get_camera_shader_data();

        // 按 Pass 渲染
        for pass in &self.pipeline.borrow().passes {
            for entity in self.get_world().borrow().get_entities().iter() {
                let entity = entity.borrow();

                // 检查这个 Entity 的 Material 是否包含当前 Pass
                let material_wrapper = entity.material_group.borrow();
                let material = match material_wrapper.materials.get(&pass.name) {
                    Some(mat) => mat.clone(),
                    None => continue, // 跳过这个 Pass
                };

                // 计算这个物体相关的数据
                let model_matrix = entity.transform.to_matrix();
                let normal_matrix = entity.transform.to_normal_matrix().unwrap();

                // 注入全局 Uniform
                let global_uniform = GlobalUniform {
                    view_matrix: &view_matrix,
                    projection_matrix: &projection_matrix,
                    view_position: &view_position,
                    model_matrix: &model_matrix,
                    normal_matrix: &normal_matrix,
                    light_count: &light_count,
                    lights_shader_data: &lights_shader_data,
                    camera_shader_data: &camera_shader_data,
                };
                material.borrow_mut().inject_global_uniform(&global_uniform);

                // 合并 RenderState
                let final_state = material.borrow().final_state(&pass.default_state);
                // 应用state
                final_state.apply();

                // 绑定 Shader
                if let Err(_) = material.borrow().bind() {
                    error!("bind material fail");
                    continue;
                }

                // 绘制 Mesh
                entity.mesh_wrapper.borrow().draw();

                // 卸载材质
                material.borrow().unbind();
            }
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
