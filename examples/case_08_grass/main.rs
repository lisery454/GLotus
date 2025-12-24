use std::error::Error;

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new_with_config(AppConfig {
        anti_pixel_msaa: AntiPixel::MSAA16,
        ..Default::default()
    });

    app.borrow().build(|context| {
        let shader = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .shader_manager
            .create_from_sources(
                include_str!("./assets/shaders/vs.vert"),
                include_str!("./assets/shaders/fs.frag"),
            )?;

        let texture = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .texture_manager
            .create_from_byte(
                include_bytes!("./assets/textures/grass.png"),
                WrappingMode::ClampToEdge,
                WrappingMode::ClampToEdge,
                FilteringMode::LinearMipmapLinear,
                FilteringMode::Linear,
            )?;

        let material = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .material_manager
            .create(shader)?;

        context
            .borrow()
            .asset_manager
            .borrow_mut()
            .material_manager
            .insert_uniform(material, "texture1", UniformValue::Texture(0, texture));

        let mesh = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .mesh_manager
            .create_from_position_texcoord(
                &vec![0, 1, 3, 1, 2, 3],
                &vec![
                    1.0, 1.0, -5.0, // 0
                    1.0, -1.0, -5.0, // 1
                    -1.0, -1.0, -5.0, // 2
                    -1.0, 1.0, -5.0, // 3
                ],
                &vec![1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
            )?;

        let context_borrow = context.borrow();
        let mut world = context_borrow.world.borrow_mut();

        let entity = world.spawn_entity();
        world.get_manager_mut::<RenderableComponent>().add(
            entity,
            RenderableComponent::new(mesh).with_material(DefaultPipeline::main_pass(), material),
        );
        world.get_manager_mut::<TransformComponent>().add(
            entity,
            TransformComponent::new(Transform::from_position(0.0, 0.0, 0.0)),
        );

        let camera_entity = world.spawn_entity();
        world
            .get_manager_mut::<CameraComponent>()
            .add(camera_entity, CameraComponent::new(true));
        world.get_manager_mut::<TransformComponent>().add(
            camera_entity,
            TransformComponent::new(Transform::from_position(0.0, 0.0, 6.0)),
        );

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
