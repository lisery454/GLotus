use cgmath::{Euler, InnerSpace, Matrix4, Quaternion, Rad, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Rotation {
    data: Quaternion<f32>,
}

impl Default for Rotation {
    fn default() -> Self {
        Rotation::zero()
    }
}

impl From<Quaternion<f32>> for Rotation {
    fn from(value: Quaternion<f32>) -> Self {
        Self { data: value }
    }
}

impl Rotation {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: Quaternion::from(Euler {
                x: Rad(x.to_radians()),
                y: Rad(y.to_radians()),
                z: Rad(z.to_radians()),
            }),
        }
    }

    pub fn zero() -> Self {
        Rotation::new(0.0, 0.0, 0.0)
    }

    pub(super) fn get_rotation_matrix(&self) -> Matrix4<f32> {
        Matrix4::from(self.data)
    }

    pub(crate) fn get_data(&self) -> Quaternion<f32> {
        self.data
    }

    pub(crate) fn rotate(&mut self, delta: Quaternion<f32>) {
        self.data = (delta * self.data).normalize();
    }

    pub fn forward(&self) -> Vector3<f32> {
        let forward = Vector3::new(0.0, 0.0, -1.0);
        (self.data * forward).normalize()
    }
}
