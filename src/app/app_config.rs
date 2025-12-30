use crate::{AntiPixel, Color, Pipeline};



pub struct AppConfig {
    pub title: String,
    pub target_render_fps: Option<u32>, // None = Unlimited
    pub fixed_update_fps: u32,          // e.g. 60
    pub v_sync: bool,
    pub anti_pixel_msaa: AntiPixel,
    pub width: u32,
    pub height: u32,
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
            anti_pixel_msaa: AntiPixel::MSAA4,
            width: 1440,
            height: 960,
            bg_color: Color::from_rgb(50, 75, 75),
            pipeline_configurer: None,
        }
    }
}
