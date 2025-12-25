use std::error::Error;

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new_with_config(AppConfig {
        anti_pixel_msaa: AntiPixel::MSAA16,
        ..Default::default()
    });

    app.borrow().build(|context| {
        let shader = context.borrow().create_shader_from_sources(
            include_str!("./assets/shaders/vs.vert"),
            include_str!("./assets/shaders/fs.frag"),
        )?;

        let texture = context.borrow().create_texture_from_byte(
            include_bytes!("./assets/textures/grass.png"),
            WrappingMode::ClampToEdge,
            WrappingMode::ClampToEdge,
            FilteringMode::LinearMipmapLinear,
            FilteringMode::Linear,
        )?;

        let material = context.borrow().create_material(shader)?;

        context.borrow().insert_uniform_to_material(
            material,
            "texture1",
            UniformValue::Texture(0, texture),
        );

        let mesh = context.borrow().create_mesh_from_position_texcoord(
            &vec![0, 1, 3, 1, 2, 3],
            &vec![
                1.0, 1.0, -5.0, // 0
                1.0, -1.0, -5.0, // 1
                -1.0, -1.0, -5.0, // 2
                -1.0, 1.0, -5.0, // 3
            ],
            &vec![1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
        )?;

        context.borrow().spawn_entity_with((
            RenderableComponent::new(mesh).with_material(DefaultPipeline::main_pass(), material),
            TransformComponent::new(Transform::from_position(0.0, 0.0, 0.0)),
        ));

        context.borrow().spawn_entity_with((
            CameraComponent::new(true),
            TransformComponent::new(Transform::from_position(0.0, 0.0, 6.0)),
        ));

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
