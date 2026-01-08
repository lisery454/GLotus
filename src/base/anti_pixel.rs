#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AntiPixel {
    MSAA32,
    MSAA16,
    MSAA8,
    MSAA4,
    MSAA2,
    NONE,
}

impl Default for AntiPixel {
    fn default() -> Self {
        AntiPixel::NONE
    }
}

impl AntiPixel {
    pub fn to_num(&self) -> Option<u32> {
        match self {
            AntiPixel::MSAA32 => Some(32),
            AntiPixel::MSAA16 => Some(16),
            AntiPixel::MSAA8 => Some(8),
            AntiPixel::MSAA4 => Some(4),
            AntiPixel::MSAA2 => Some(2),
            AntiPixel::NONE => None,
        }
    }

    pub fn samples(&self) -> u32 {
        match self {
            AntiPixel::MSAA32 => 32,
            AntiPixel::MSAA16 => 16,
            AntiPixel::MSAA8 => 8,
            AntiPixel::MSAA4 => 4,
            AntiPixel::MSAA2 => 2,
            AntiPixel::NONE => 1,
        }
    }
}
