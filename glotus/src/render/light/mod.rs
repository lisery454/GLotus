pub mod directional_light;
pub mod point_light;
pub mod spot_light;
pub mod light;

pub use light::{Light, LightType, LightShaderData};
pub use point_light::PointLight;
pub use spot_light::SpotLight;
pub use directional_light::DirectionalLight;