/// **颜色**
#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    /// 从rgb三维生成，默认是0到255范围之间
    pub fn from_rgb(r: u32, g: u32, b: u32) -> Self {
        let r = r.clamp(0, 255);
        let g = g.clamp(0, 255);
        let b = b.clamp(0, 255);

        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;

        Self { r, g, b, a: 1.0 }
    }

    /// 返回rgb三维数组的值，范围在0~1
    pub fn to_arr(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    /// 从hsv三维生成，h是0~1，s
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
            a: 1.0,
        }
    }
}

// some color prefab
impl Color {
    /// 内部使用的 const 构造函数，绕过编译期无法浮点运算的限制
    const fn new_const(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    // --- 灰阶色 (Greyscale) ---
    pub const ALICE_BLUE: Self = Self::new_const(0.94118, 0.97255, 1.0);
    pub const ANTIQUE_WHITE: Self = Self::new_const(0.98039, 0.92157, 0.84314);
    pub const AZURE: Self = Self::new_const(0.94118, 1.0, 1.0);
    pub const BEIGE: Self = Self::new_const(0.96078, 0.96078, 0.86275);
    pub const BISQUE: Self = Self::new_const(1.0, 0.89412, 0.76863);
    pub const BLACK: Self = Self::new_const(0.0, 0.0, 0.0);
    pub const BLANCHED_ALMOND: Self = Self::new_const(1.0, 0.92157, 0.80392);
    pub const CORNSILK: Self = Self::new_const(1.0, 0.97255, 0.86275);
    pub const DARK_GRAY: Self = Self::new_const(0.66275, 0.66275, 0.66275);
    pub const DARK_SLATE_GRAY: Self = Self::new_const(0.18431, 0.30980, 0.30980);
    pub const DIM_GRAY: Self = Self::new_const(0.41176, 0.41176, 0.41176);
    pub const GAINSBORO: Self = Self::new_const(0.86275, 0.86275, 0.86275);
    pub const GHOST_WHITE: Self = Self::new_const(0.97255, 0.97255, 1.0);
    pub const GRAY: Self = Self::new_const(0.50196, 0.50196, 0.50196);
    pub const HONEYDEW: Self = Self::new_const(0.94118, 1.0, 0.94118);
    pub const IVORY: Self = Self::new_const(1.0, 1.0, 0.94118);
    pub const LIGHT_GRAY: Self = Self::new_const(0.82745, 0.82745, 0.82745);
    pub const LIGHT_SLATE_GRAY: Self = Self::new_const(0.46667, 0.53333, 0.6);
    pub const LINEN: Self = Self::new_const(0.98039, 0.94118, 0.90196);
    pub const OLD_LACE: Self = Self::new_const(0.99216, 0.96078, 0.90196);
    pub const SILVER: Self = Self::new_const(0.75294, 0.75294, 0.75294);
    pub const SLATE_GRAY: Self = Self::new_const(0.43922, 0.50196, 0.56471);
    pub const SNOW: Self = Self::new_const(1.0, 0.98039, 0.98039);
    pub const WHITE: Self = Self::new_const(1.0, 1.0, 1.0);
    pub const WHITE_SMOKE: Self = Self::new_const(0.96078, 0.96078, 0.96078);

    // --- 红色与粉色系 (Reds & Pinks) ---
    pub const CRIMSON: Self = Self::new_const(0.86275, 0.07843, 0.23529);
    pub const DARK_RED: Self = Self::new_const(0.54510, 0.0, 0.0);
    pub const DEEP_PINK: Self = Self::new_const(1.0, 0.07843, 0.57647);
    pub const FIRE_BRICK: Self = Self::new_const(0.69804, 0.13333, 0.13333);
    pub const HOT_PINK: Self = Self::new_const(1.0, 0.41176, 0.70588);
    pub const INDIAN_RED: Self = Self::new_const(0.80392, 0.36078, 0.36078);
    pub const LIGHT_CORAL: Self = Self::new_const(0.94118, 0.50196, 0.50196);
    pub const LIGHT_PINK: Self = Self::new_const(1.0, 0.71373, 0.75686);
    pub const MISTY_ROSE: Self = Self::new_const(1.0, 0.89412, 0.88235);
    pub const PALE_VIOLET_RED: Self = Self::new_const(0.85882, 0.43922, 0.57647);
    pub const PINK: Self = Self::new_const(1.0, 0.75294, 0.79608);
    pub const RED: Self = Self::new_const(1.0, 0.0, 0.0);
    pub const ROSY_BROWN: Self = Self::new_const(0.73725, 0.56078, 0.56078);
    pub const SALMON: Self = Self::new_const(0.98039, 0.50196, 0.44706);

    // --- 橙色与黄色系 (Oranges & Yellows) ---
    pub const CORAL: Self = Self::new_const(1.0, 0.49804, 0.31373);
    pub const DARK_ORANGE: Self = Self::new_const(1.0, 0.54902, 0.0);
    pub const DARK_SALMON: Self = Self::new_const(0.91373, 0.58824, 0.47843);
    pub const GOLD: Self = Self::new_const(1.0, 0.84314, 0.0);
    pub const GOLDENROD: Self = Self::new_const(0.85490, 0.64706, 0.12549);
    pub const LIGHT_SALMON: Self = Self::new_const(1.0, 0.62745, 0.47843);
    pub const LIGHT_YELLOW: Self = Self::new_const(1.0, 1.0, 0.87843);
    pub const MOCCASIN: Self = Self::new_const(1.0, 0.89412, 0.70980);
    pub const ORANGE: Self = Self::new_const(1.0, 0.64706, 0.0);
    pub const ORANGE_RED: Self = Self::new_const(1.0, 0.27059, 0.0);
    pub const PALE_GOLDENROD: Self = Self::new_const(0.93333, 0.90980, 0.66667);
    pub const PAPAYA_WHIP: Self = Self::new_const(1.0, 0.93725, 0.83529);
    pub const PEACH_PUFF: Self = Self::new_const(1.0, 0.85490, 0.72549);
    pub const TOMATO: Self = Self::new_const(1.0, 0.38824, 0.27843);
    pub const YELLOW: Self = Self::new_const(1.0, 1.0, 0.0);

    // --- 绿色系 (Greens) ---
    pub const CHARTREUSE: Self = Self::new_const(0.49804, 1.0, 0.0);
    pub const DARK_GREEN: Self = Self::new_const(0.0, 0.39216, 0.0);
    pub const DARK_OLIVE_GREEN: Self = Self::new_const(0.33333, 0.41961, 0.18431);
    pub const DARK_SEA_GREEN: Self = Self::new_const(0.56078, 0.73725, 0.56078);
    pub const FOREST_GREEN: Self = Self::new_const(0.13333, 0.54510, 0.13333);
    pub const GREEN: Self = Self::new_const(0.0, 0.50196, 0.0);
    pub const GREEN_YELLOW: Self = Self::new_const(0.67843, 1.0, 0.18431);
    pub const LAWN_GREEN: Self = Self::new_const(0.48627, 0.98824, 0.0);
    pub const LIGHT_GREEN: Self = Self::new_const(0.56471, 0.93333, 0.56471);
    pub const LIME: Self = Self::new_const(0.0, 1.0, 0.0);
    pub const LIME_GREEN: Self = Self::new_const(0.19608, 0.80392, 0.19608);
    pub const MEDIUM_SEA_GREEN: Self = Self::new_const(0.23529, 0.70196, 0.44314);
    pub const MEDIUM_SPRING_GREEN: Self = Self::new_const(0.0, 0.98039, 0.60392);
    pub const OLIVE: Self = Self::new_const(0.50196, 0.50196, 0.0);
    pub const OLIVE_DRAB: Self = Self::new_const(0.41961, 0.55686, 0.13725);
    pub const PALE_GREEN: Self = Self::new_const(0.59608, 0.98431, 0.59608);
    pub const SEA_GREEN: Self = Self::new_const(0.18039, 0.54510, 0.34118);
    pub const SPRING_GREEN: Self = Self::new_const(0.0, 1.0, 0.49804);
    pub const YELLOW_GREEN: Self = Self::new_const(0.60392, 0.80392, 0.19608);

    // --- 蓝色与青色系 (Blues & Cyans) ---
    pub const AQUA: Self = Self::new_const(0.0, 1.0, 1.0);
    pub const AQUAMARINE: Self = Self::new_const(0.49804, 1.0, 0.83137);
    pub const BLUE: Self = Self::new_const(0.0, 0.0, 1.0);
    pub const BLUE_VIOLET: Self = Self::new_const(0.54118, 0.16863, 0.88627);
    pub const CADET_BLUE: Self = Self::new_const(0.37255, 0.61961, 0.62745);
    pub const CORNFLOWER_BLUE: Self = Self::new_const(0.39216, 0.58431, 0.92941);
    pub const CYAN: Self = Self::new_const(0.0, 1.0, 1.0);
    pub const DARK_BLUE: Self = Self::new_const(0.0, 0.0, 0.54510);
    pub const DARK_CYAN: Self = Self::new_const(0.0, 0.54510, 0.54510);
    pub const DARK_TURQUOISE: Self = Self::new_const(0.0, 0.80784, 0.81961);
    pub const DEEP_SKY_BLUE: Self = Self::new_const(0.0, 0.74902, 1.0);
    pub const DODGER_BLUE: Self = Self::new_const(0.11765, 0.56471, 1.0);
    pub const LIGHT_BLUE: Self = Self::new_const(0.67843, 0.84706, 0.90196);
    pub const LIGHT_CYAN: Self = Self::new_const(0.87843, 1.0, 1.0);
    pub const LIGHT_SKY_BLUE: Self = Self::new_const(0.52941, 0.80784, 0.98039);
    pub const LIGHT_STEEL_BLUE: Self = Self::new_const(0.69020, 0.76863, 0.87059);
    pub const MEDIUM_BLUE: Self = Self::new_const(0.0, 0.0, 0.80392);
    pub const MEDIUM_TURQUOISE: Self = Self::new_const(0.28235, 0.81961, 0.8);
    pub const MIDNIGHT_BLUE: Self = Self::new_const(0.09804, 0.09804, 0.43922);
    pub const NAVY: Self = Self::new_const(0.0, 0.0, 0.50196);
    pub const POWDER_BLUE: Self = Self::new_const(0.69020, 0.87843, 0.90196);
    pub const ROYAL_BLUE: Self = Self::new_const(0.25490, 0.41176, 0.88235);
    pub const SKY_BLUE: Self = Self::new_const(0.52941, 0.80784, 0.92157);
    pub const STEEL_BLUE: Self = Self::new_const(0.27451, 0.50980, 0.70588);
    pub const TEAL: Self = Self::new_const(0.0, 0.50196, 0.50196);
    pub const TURQUOISE: Self = Self::new_const(0.25098, 0.87843, 0.81569);

    // --- 紫色系 (Purples) ---
    pub const DARK_MAGENTA: Self = Self::new_const(0.54510, 0.0, 0.54510);
    pub const DARK_ORCHID: Self = Self::new_const(0.6, 0.19608, 0.8);
    pub const DARK_SLATE_BLUE: Self = Self::new_const(0.28235, 0.23922, 0.54510);
    pub const DARK_VIOLET: Self = Self::new_const(0.58039, 0.0, 0.82745);
    pub const FUCHSIA: Self = Self::new_const(1.0, 0.0, 1.0);
    pub const INDIGO: Self = Self::new_const(0.29412, 0.0, 0.50980);
    pub const LAVENDER: Self = Self::new_const(0.90196, 0.90196, 0.98039);
    pub const MAGENTA: Self = Self::new_const(1.0, 0.0, 1.0);
    pub const MEDIUM_ORCHID: Self = Self::new_const(0.72941, 0.33333, 0.82745);
    pub const MEDIUM_PURPLE: Self = Self::new_const(0.57647, 0.43922, 0.85882);
    pub const MEDIUM_SLATE_BLUE: Self = Self::new_const(0.48235, 0.40784, 0.93333);
    pub const ORCHID: Self = Self::new_const(0.85490, 0.43922, 0.83922);
    pub const PLUM: Self = Self::new_const(0.86667, 0.62745, 0.86667);
    pub const PURPLE: Self = Self::new_const(0.50196, 0.0, 0.50196);
    pub const REBECCA_PURPLE: Self = Self::new_const(0.4, 0.2, 0.6);
    pub const SLATE_BLUE: Self = Self::new_const(0.41569, 0.35294, 0.80392);
    pub const THISTLE: Self = Self::new_const(0.84706, 0.74902, 0.84706);
    pub const VIOLET: Self = Self::new_const(0.93333, 0.50980, 0.93333);

    // --- 棕色与大地色 (Browns & Earth Tones) ---
    pub const BROWN: Self = Self::new_const(0.64706, 0.16471, 0.16471);
    pub const BURLY_WOOD: Self = Self::new_const(0.87059, 0.72157, 0.52941);
    pub const CHOCOLATE: Self = Self::new_const(0.82353, 0.41176, 0.11765);
    pub const DARK_GOLDENROD: Self = Self::new_const(0.72157, 0.52549, 0.04314);
    pub const DARK_KHAKI: Self = Self::new_const(0.74118, 0.71765, 0.41961);
    pub const KHAKI: Self = Self::new_const(0.94118, 0.90196, 0.54902);
    pub const MAROON: Self = Self::new_const(0.50196, 0.0, 0.0);
    pub const PERU: Self = Self::new_const(0.80392, 0.52157, 0.24706);
    pub const SADDLE_BROWN: Self = Self::new_const(0.54510, 0.27059, 0.07451);
    pub const SANDY_BROWN: Self = Self::new_const(0.95686, 0.64314, 0.37647);
    pub const SIENNA: Self = Self::new_const(0.62745, 0.32157, 0.17647);
    pub const TAN: Self = Self::new_const(0.82353, 0.70588, 0.54902);
    pub const WHEAT: Self = Self::new_const(0.96078, 0.87059, 0.70196);

    pub const TRANSPARENT: Self = Self {
        a: 0.0,
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };
}

impl Default for Color {
    /// 默认是白色
    fn default() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
}
