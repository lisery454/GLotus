use crate::{Camera, Light, LightData, ProjectionType, Transform, TransformError};

/// 相机的shader数据，用来传递给shader
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct CameraShaderData {
    //
    pub position: [f32; 4], // 16B: xyz=坐标, w=1.0
    //
    pub direction: [f32; 4], // 16B: xyz=向量, w=0.0
    //
    pub camera_type: i32,  // 4B
    pub fov: f32,          // 4B
    pub aspect_ratio: f32, // 4B
    pub near_plane: f32,   // 4B
    //
    pub far_plane: f32, // 4B
    pub _pad: [f32; 3], // 12B
}

/// 光源shader数据
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct LightShaderData {
    // 16B
    pub light_type: i32,
    pub _pad1: [i32; 3],
    // 16B
    pub color: [f32; 4],
    // 16B
    pub position: [f32; 4],
    // 16B
    pub direction: [f32; 4],
    // 16B
    pub intensity: f32,
    pub range: f32,
    pub inner_cone: f32,
    pub outer_cone: f32,
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct FrameData {
    // 16B
    pub light_count: i32,
    pub _pad1: [i32; 3],
    // 16 * 80B = 1280B (每个 Light 80 字节)
    pub lights: [LightShaderData; 16],
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct CameraData {
    // 64B (mat4 = 4 * vec4 = 4 * 16B)
    pub view_matrix: [[f32; 4]; 4],
    // 64B
    pub projection_matrix: [[f32; 4]; 4],
    // 64B
    pub camera: CameraShaderData,
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct ModelData {
    // 64B (mat4 = 4 * vec4)
    pub model_matrix: [[f32; 4]; 4],
    // 64B (mat4 = 4 * vec4)
    pub normal_matrix: [[f32; 4]; 4],
}

impl Default for CameraShaderData {
    fn default() -> Self {
        Self {
            camera_type: Default::default(),
            fov: Default::default(),
            direction: Default::default(),
            position: Default::default(),
            aspect_ratio: Default::default(),
            near_plane: Default::default(),
            far_plane: Default::default(),
            _pad: Default::default(),
        }
    }
}

impl CameraShaderData {
    pub fn new(
        camera_type: i32,
        fov: f32,
        direction: [f32; 4],
        position: [f32; 4],
        aspect_ratio: f32,
        near_plane: f32,
        far_plane: f32,
    ) -> Self {
        Self {
            camera_type,
            fov,
            direction,
            position,
            aspect_ratio,
            near_plane,
            far_plane,
            _pad: Default::default(),
        }
    }

    pub fn from_camera(camera: &Camera, transform: &Transform) -> Self {
        let [x, y, z] = transform.get_rotation().forward().into();
        let direction = [x, y, z, 0.0];
        let [x, y, z] = transform.get_translation().get_arr().into();
        let position = [x, y, z, 1.0];
        Self::new(
            if camera.projection_type == ProjectionType::Perspective {
                0
            } else {
                1
            },
            camera.fov.0,
            direction,
            position,
            camera.aspect_ratio,
            camera.near_plane,
            camera.far_plane,
        )
    }
}

impl Default for LightShaderData {
    fn default() -> Self {
        Self {
            light_type: Default::default(),
            _pad1: Default::default(),
            color: Default::default(),
            position: Default::default(),
            direction: Default::default(),
            intensity: Default::default(),
            range: Default::default(),
            inner_cone: Default::default(),
            outer_cone: Default::default(),
        }
    }
}

impl LightShaderData {
    pub fn new(
        light_type: i32,
        color: [f32; 4],
        position: [f32; 4],
        direction: [f32; 4],
        intensity: f32,
        range: f32,
        inner_cone: f32,
        outer_cone: f32,
    ) -> Self {
        Self {
            light_type,
            _pad1: [0; 3],
            color,
            position,
            direction,
            intensity,
            range,
            inner_cone,
            outer_cone,
        }
    }

    pub fn from_light(light: &Light, transform: &Transform) -> Self {
        // 预提取通用属性
        let color = light.color.to_arr();
        let intensity = light.intensity;
        let [x, y, z] = transform.get_translation().get_arr();
        let position = [x, y, z, 1.0];
        let [x, y, z] = transform.get_rotation().forward().into();
        let direction = [x, y, z, 0.0];

        match light.data {
            LightData::Directional => Self::new(
                0,
                color,
                [0.0, 0.0, 0.0, 1.0],
                direction,
                intensity,
                0.0,
                0.0,
                0.0,
            ),
            LightData::Point { range } => {
                Self::new(1, color, position, [0.0; 4], intensity, range, 0.0, 0.0)
            }
            LightData::Spot {
                range,
                inner,
                outer,
            } => Self::new(
                2, color, position, direction, intensity, range, inner, outer,
            ),
        }
    }
}

impl Default for FrameData {
    fn default() -> Self {
        Self {
            light_count: Default::default(),
            _pad1: Default::default(),
            lights: Default::default(),
        }
    }
}

impl FrameData {
    pub fn new(lights: &Vec<LightShaderData>) -> Self {
        let mut frame_data = Self::default();
        let count = lights.len().min(16);
        frame_data.light_count = count as i32;
        for i in 0..count {
            frame_data.lights[i] = lights[i].clone();
        }
        frame_data
    }
}

impl Default for CameraData {
    fn default() -> Self {
        Self {
            view_matrix: Default::default(),
            projection_matrix: Default::default(),
            camera: Default::default(),
        }
    }
}

impl CameraData {
    pub fn new(camera: &Camera, camera_transform: &Transform) -> Self {
        let view_matrix = camera_transform.get_view_matrix().into();
        let projection_matrix = camera.get_projection_matrix();
        let camera_shader_data = CameraShaderData::from_camera(camera, camera_transform);
        Self {
            view_matrix,
            projection_matrix,
            camera: camera_shader_data,
        }
    }
}

impl Default for ModelData {
    fn default() -> Self {
        Self {
            model_matrix: Default::default(),
            normal_matrix: Default::default(),
        }
    }
}

impl ModelData {
    pub fn new(model_transform: &Transform) -> Result<Self, TransformError> {
        let model_matrix = model_transform.to_matrix();
        let normal_matrix = model_transform.to_normal_matrix()?;

        let mut dm = Self::default();
        dm.model_matrix = model_matrix;
        dm.normal_matrix = normal_matrix;
        Ok(dm)
    }
}
