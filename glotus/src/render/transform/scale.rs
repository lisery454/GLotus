use cgmath::{Matrix4, Vector3};

#[derive(Debug)]
pub struct Scale {
    data: Vector3<f32>,
}

impl Default for Scale {
    fn default() -> Self {
        Scale::one()
    }
}

impl Scale {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: Vector3::new(x, y, z),
        }
    }

    pub fn one() -> Self {
        Self {
            data: Vector3::new(1.0, 1.0, 1.0),
        }
    }

    pub(super) fn get_scale_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_nonuniform_scale(self.data.x, self.data.y, self.data.z)
    }

    pub(crate) fn get_data(&self) -> Vector3<f32> {
        self.data
    }
}
