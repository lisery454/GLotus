#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum PolygonMode {
    Fill,
    Line,
    Point,
}

impl Default for PolygonMode {
    fn default() -> Self {
        PolygonMode::Fill
    }
}

impl PolygonMode {
    pub fn apply(&self) {
        unsafe {
            match self {
                PolygonMode::Fill => gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL),
                PolygonMode::Line => gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE),
                PolygonMode::Point => gl::PolygonMode(gl::FRONT_AND_BACK, gl::POINT),
            }
        }
    }
}
