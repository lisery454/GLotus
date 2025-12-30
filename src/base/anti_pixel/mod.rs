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
