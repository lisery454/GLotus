#[derive(Debug)]
pub enum WrappingMode {
    Repeat,
    MirroreroredRepeat,
    ClampToEdge,
    ClampToBorder { color: [f32; 4] },
}

#[derive(Debug)]
pub enum FilteringMode {
    Nearest,
    Linear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear,
}