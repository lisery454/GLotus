use crate::{AntiPixel, Color, Pipeline, Resolution};

pub struct AppConfig {
    pub title: String,
    pub target_render_fps: Option<u32>, // None = Unlimited
    pub fixed_update_fps: u32,          // e.g. 60
    pub v_sync: bool,
    pub anti_pixel: AntiPixel,
    pub resolution: Resolution,
    pub bg_color: Color,
    pub pipeline_configurer: Option<Box<dyn Fn(&mut Pipeline)>>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            title: String::from("Rust GLFW opengl"),
            target_render_fps: None,
            fixed_update_fps: 60,
            v_sync: true,
            anti_pixel: AntiPixel::MSAA4,
            resolution: Resolution::new(1440, 960),
            bg_color: Color::from_rgb(50, 75, 75),
            pipeline_configurer: None,
        }
    }
}
