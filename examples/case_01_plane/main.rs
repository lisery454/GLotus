use std::collections::HashMap;
use std::error::Error;

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new();

    app.borrow().build(|context| {
        let shader_handle = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .shader_manager
            .create_from_sources(
                include_str!("./assets/shaders/vs.vert"),
                include_str!("./assets/shaders/fs.frag"),
            )?;

        let material_handle = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .material_manager
            .create(shader_handle)?;

        let pass_name = DefaultPipeline::get_default_pass_name();

        let mesh_handle = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .mesh_manager
            .create_from_position(
                &vec![0, 1, 3, 1, 2, 3],
                &vec![
                    1.0, 1.0, -5.0, // 0
                    1.0, -1.0, -5.0, // 1
                    -1.0, -1.0, -5.0, // 2
                    -1.0, 1.0, -5.0, // 3
                ],
            )?;

        let context_borrow = context.borrow();
        let mut world = context_borrow.world.borrow_mut();
        let entity = world.spawn_entity();
        world.get_manager_mut::<RenderableComponent>().add(
            entity,
            RenderableComponent::new(HashMap::from([(pass_name, material_handle)]), mesh_handle),
        );
        world
            .get_manager_mut::<TransformComponent>()
            .add(entity, TransformComponent::new(Transform::default()));
        let camera_entity = world.spawn_entity();
        world
            .get_manager_mut::<CameraComponent>()
            .add(camera_entity, CameraComponent::new(true));
        world.get_manager_mut::<TransformComponent>().add(
            camera_entity,
            TransformComponent::new(Transform::from_position(0.0, 0.0, 10.0)),
        );

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
