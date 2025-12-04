pub enum AntiPixel {
    MSAA32,
    MSAA16,
    MSAA8,
    MSAA4,
    NONE,
}

impl AntiPixel {
    pub fn to_num(&self) -> Option<u32> {
        match self {
            AntiPixel::MSAA32 => Some(32),
            AntiPixel::MSAA16 => Some(16),
            AntiPixel::MSAA8 => Some(8),
            AntiPixel::MSAA4 => Some(4),
            AntiPixel::NONE => None,
        }
    }
}

pub struct AppConfig {
    pub title: String,
    pub target_render_fps: Option<u32>, // None = Unlimited
    pub fixed_update_fps: u32,          // e.g. 60
    pub v_sync: bool,
    pub anti_pixel_msaa: AntiPixel,
    pub width: u32,
    pub height: u32,
    pub bg_color: [f32; 3],
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            title: String::from("Rust GLFW opengl"),
            target_render_fps: None,
            fixed_update_fps: 60,
            v_sync: true,
            anti_pixel_msaa: AntiPixel::MSAA4,
            width: 1440,
            height: 960,
            bg_color: [0.2, 0.3, 0.3],
        }
    }
}
