#[derive(Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn from_rgb(r: u32, g: u32, b: u32) -> Self {
        let r = r.clamp(0, 255);
        let g = g.clamp(0, 255);
        let b = b.clamp(0, 255);

        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;

        Self { r, g, b }
    }

    pub fn to_arr(&self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }

    pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        let h = (h * 360.0) % 360.0;
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r1, g1, b1) = match h as i32 {
            0..=60 => (c, x, 0.0),
            61..=120 => (x, c, 0.0),
            121..=180 => (0.0, c, x),
            181..=240 => (0.0, x, c),
            241..=300 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        Color {
            r: r1 + m,
            g: g1 + m,
            b: b1 + m,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }
}
