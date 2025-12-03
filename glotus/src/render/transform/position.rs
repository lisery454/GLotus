use cgmath::{EuclideanSpace, Matrix4, Point3, Vector3};

#[derive(Debug)]
pub struct Position {
    data: Point3<f32>,
}

impl Default for Position {
    fn default() -> Self {
        Position::zero()
    }
}

impl From<Position> for [f32; 3] {
    fn from(value: Position) -> Self {
        [value.data.x, value.data.y, value.data.z]
    }
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: Point3::new(x, y, z),
        }
    }

    pub fn zero() -> Self {
        Self {
            data: Point3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn get_arr(&self) -> [f32; 3] {
        return [self.data.x, self.data.y, self.data.z];
    }

    pub(super) fn get_translation_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.data.to_vec())
    }

    pub(crate) fn get_data(&self) -> Point3<f32> {
        self.data
    }

    pub(crate) fn translate(&mut self, delta: Vector3<f32>) {
        self.data += delta;
    }
}
