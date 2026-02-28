use std::error::Error;

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new_with_config(AppConfig {
        anti_pixel: AntiPixel::MSAA16,
        ..Default::default()
    });

    app.borrow().build(|context| {
        let shader = context.borrow().with_sdr_mgr(|m| {
            m.create(ShaderConfig::new_vert_frag(
                ShaderInput::Source(include_str!("./assets/shaders/vs.vert").to_string()),
                ShaderInput::Source(include_str!("./assets/shaders/fs.frag").to_string()),
            ))
        })?;

        let texture = context.borrow().with_tex_mgr(|m| {
            m.create_from_bytes(
                include_bytes!("./assets/textures/wood.png"),
                TextureConfig::common(
                    WrappingMode::Repeat,
                    WrappingMode::Repeat,
                    FilteringMode::LinearMipmapLinear,
                    FilteringMode::Linear,
                    FormatType::SRGBA,
                ),
            )
        })?;

        let material = context.borrow().with_mat_mgr(|m| {
            m.get_builder(shader)?
                .with("material.texture", UniformValue::Texture(0, texture))
                .with(
                    "material.ambient_factor",
                    UniformValue::Vector3([0.05, 0.05, 0.05]),
                )
                .with(
                    "material.diffuse_factor",
                    UniformValue::Vector3([1.0, 1.0, 1.0]),
                )
                .with(
                    "material.specular_factor",
                    UniformValue::Vector3([1.6, 1.6, 1.6]),
                )
                .with("material.specular_shininess", UniformValue::Float(1.0))
                .build()
        })?;

        let mesh = context.borrow().with_msh_mgr(|m| {
            m.create_from_obj_bytes(include_bytes!("./assets/meshes/plane.obj"))
        })?;

        context.borrow().with_world(|w| {
            w.spawn_entity_with((
                Renderable::new(mesh).with_material(DefaultPipeline::main_pass(), material),
                Transform::new(
                    Translation::default(),
                    Rotation::default(),
                    Scaling::new(20.0, 20.0, 20.0),
                ),
            ));

            w.spawn_entity_with((Transform::from_position(0.0, 1.0, 1.0), Camera::new(true)));

            w.spawn_entity_with((
                Light::point()
                    .with_color(Color::WHITE)
                    .with_intensity(2.0)
                    .with_range(20.0),
                Transform::from_position(0.0, 7.0, 0.0),
            ));
        });

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
