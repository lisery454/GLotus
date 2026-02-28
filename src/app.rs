mod app_config;

pub use app_config::AppConfig;

use crate::{
    AppContext, AppEvent, Resolution, SystemDispatcher,
    utils::{self},
};
use glfw::{
    Action, Context, Glfw, GlfwReceiver, Key, PWindow, SwapInterval, WindowEvent, ffi::glfwGetTime,
};
use log::{error, info};
use std::{cell::RefCell, error::Error, rc::Rc, time::Duration};

pub struct App {
    is_running: bool,
    window: Option<Rc<RefCell<PWindow>>>,
    glfw: Option<Rc<RefCell<Glfw>>>,
    event_receiver: Option<Rc<RefCell<GlfwReceiver<(f64, WindowEvent)>>>>,

    context: Rc<RefCell<AppContext>>,
    system_dispatcher: Rc<RefCell<SystemDispatcher>>,
}

// main
impl App {
    pub fn new() -> Rc<RefCell<Self>> {
        Self::new_with_config(Default::default())
    }

    pub fn new_with_config(config: AppConfig) -> Rc<RefCell<Self>> {
        let app = Self {
            is_running: false,
            window: None,
            glfw: None,
            event_receiver: None,
            context: Rc::new(RefCell::new(AppContext::new(config))),
            system_dispatcher: Rc::new(RefCell::new(SystemDispatcher::new_with_default_systems())),
        };

        utils::setup_logger();
        let app_rc = Rc::new(RefCell::new(app));
        app_rc.borrow_mut().init();
        app_rc
    }

    pub fn build<F>(&self, action: F) -> Result<(), Box<dyn Error>>
    where
        F: Fn(Rc<RefCell<AppContext>>) -> Result<(), Box<dyn Error>>,
    {
        let result = action(self.context.clone());

        result
    }

    pub fn run(&mut self) {
        self.is_running = true;

        info!("app starts to running...");

        let fixed_dt: f32;
        let target_render_dt: Option<f32>;
        {
            let context = self.context.borrow();
            let config = context.app_config.borrow();
            fixed_dt = 1.0 / config.fixed_update_fps as f32;
            target_render_dt = config.target_render_fps.map(|fps| 1.0 / fps as f32);
        }

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
            let delta_dt = self.get_current_time() - last_render_update_time;
            //  更新记录的时间
            last_render_update_time = self.get_current_time();

            self.render_update(delta_dt);

            self.window.as_ref().unwrap().borrow_mut().swap_buffers();

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

            // 清空事件队列
            {
                self.context.borrow().event_queue.borrow_mut().clear();
            }
        }

        info!("app is going to close...");
    }
}

// utils
impl App {
    fn init(&mut self) {
        let context = self.context.borrow();
        let config = context.app_config.borrow();
        let width = config.resolution.width;
        let height = config.resolution.height;
        // 初始化 GLFW
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        // 设置窗口提示
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        // 设置采样
        glfw.window_hint(glfw::WindowHint::Samples(config.anti_pixel.to_num()));

        // 创建窗口
        let (mut window, events) = glfw
            .create_window(
                width,
                height,
                config.title.as_str(),
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
        if config.v_sync {
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
        if config.anti_pixel.to_num().is_some() {
            unsafe {
                gl::Enable(gl::MULTISAMPLE);
            }
        }

        unsafe {
            gl::Enable(gl::FRAMEBUFFER_SRGB);
        }

        // 初始化视口
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }

        // 初始化system
        if let Err(e) = self
            .system_dispatcher
            .borrow_mut()
            .init_systems(self.context.clone())
        {
            error!("init system error: {}", e);
        };
    }

    fn handle_event_queue(&mut self) {
        let mut need_close = false;

        {
            let context = self.context.borrow();
            let event_queue = context.event_queue.borrow();
            let events = event_queue.all_events();
            let mut input_state = context.input_state.borrow_mut();

            for event in events {
                match event {
                    AppEvent::Resize { width, height } => unsafe {
                        gl::Viewport(0, 0, *width, *height);
                    },
                    AppEvent::Key { key, action } => match action {
                        Action::Press => {
                            input_state.press_key(*key);
                        }
                        Action::Release => {
                            input_state.release_key(key);
                        }
                        _ => {}
                    },
                    AppEvent::Close => {
                        need_close = true;
                    }
                    AppEvent::Scroll { x, y } => {
                        input_state.set_scroll_delta(*x as f32, *y as f32);
                    }
                    AppEvent::CursorPos { x, y } => {
                        input_state.set_cursor_delta(*x as f32, *y as f32);
                    }
                    AppEvent::MouseButton { button, action } => match action {
                        Action::Press => {
                            input_state.press_mouse_button(*button);
                        }
                        Action::Release => {
                            input_state.release_mouse_button(button);
                        }
                        _ => {}
                    },
                }
            }
        }

        if need_close {
            self.close();
        }
    }

    fn fixed_update(&mut self, fixed_dt: f32) {
        // esc 自动退出app
        if self
            .context
            .borrow()
            .input_state
            .borrow()
            .is_key_down(Key::Escape)
        {
            self.window
                .as_ref()
                .unwrap()
                .borrow_mut()
                .set_should_close(true);
            self.is_running = false;
        }

        if let Err(e) = self
            .system_dispatcher
            .borrow_mut()
            .fixed_run_systems(self.context.clone(), fixed_dt)
        {
            error!("fixed run system error: {}", e);
        };

        self.context.borrow().input_state.borrow_mut().clear_delta();
    }

    fn get_current_time(&self) -> f32 {
        unsafe {
            let current_time = glfwGetTime() as f32;
            current_time
        }
    }

    fn render_update(&mut self, delta_dt: f32) {
        if let Err(e) = self
            .system_dispatcher
            .borrow_mut()
            .run_systems(self.context.clone(), delta_dt)
        {
            error!("render update system error: {}", e);
        };
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
        let context = self.context.borrow();
        let mut event_queue = context.event_queue.borrow_mut();
        let mut window_state = context.window_state.borrow_mut();
        for (_, event) in
            glfw::flush_messages(&mut *(self.event_receiver.as_ref().unwrap().borrow_mut()))
        {
            match event {
                WindowEvent::Key(key, _, action, _) => {
                    event_queue.push(AppEvent::Key { key, action });
                }
                WindowEvent::Close => {
                    event_queue.push(AppEvent::Close);
                }
                WindowEvent::Scroll(xoffset, yoffset) => {
                    event_queue.push(AppEvent::Scroll {
                        x: xoffset,
                        y: yoffset,
                    });
                }
                WindowEvent::CursorPos(xpos, ypos) => {
                    event_queue.push(AppEvent::CursorPos { x: xpos, y: ypos });
                }
                WindowEvent::MouseButton(button, action, _) => {
                    event_queue.push(AppEvent::MouseButton { button, action });
                }
                WindowEvent::FramebufferSize(width, height) => {
                    window_state.set_resolution(Resolution::new(width as u32, height as u32));
                    event_queue.push(AppEvent::Resize { width, height });
                }
                _ => (),
            };
        }
    }
}
