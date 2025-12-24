use std::error::Error;

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new_with_config(AppConfig {
        anti_pixel_msaa: AntiPixel::MSAA16,
        ..Default::default()
    });

    app.borrow().build(|context| {
        let shader_1 = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .shader_manager
            .create_from_sources(
                include_str!("./assets/shaders/vs_1.vert"),
                include_str!("./assets/shaders/fs_1.frag"),
            )?;

        let shader_2 = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .shader_manager
            .create_from_sources(
                include_str!("./assets/shaders/vs_2.vert"),
                include_str!("./assets/shaders/fs_2.frag"),
            )?;

        let material_1 = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .material_manager
            .create(shader_1)?;

        let material_2 = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .material_manager
            .create(shader_2)?;

        let mesh = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .mesh_manager
            .create_from_obj_in_bytes(include_bytes!("./assets/meshes/sphere.obj"))?;

        let mesh2 = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .mesh_manager
            .create_from_obj_in_bytes(include_bytes!("./assets/meshes/sphere_no_smooth.obj"))?;

        let mesh3 = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .mesh_manager
            .create_from_obj_in_bytes(include_bytes!("./assets/meshes/box.obj"))?;

        let context_borrow = context.borrow();
        let mut world = context_borrow.world.borrow_mut();

        let entity = world.spawn_entity();
        world.get_manager_mut::<RenderableComponent>().add(
            entity,
            RenderableComponent::new(mesh)
                .with_material(DefaultPipeline::main_pass(), material_1)
                .with_material(DefaultPipeline::outline_pass(), material_2),
        );
        world.get_manager_mut::<TransformComponent>().add(
            entity,
            TransformComponent::new(Transform::from_position(0.0, 0.0, 0.0)),
        );

        let entity = world.spawn_entity();
        world.get_manager_mut::<RenderableComponent>().add(
            entity,
            RenderableComponent::new(mesh2)
                .with_material(DefaultPipeline::main_pass(), material_1)
                .with_material(DefaultPipeline::outline_pass(), material_2),
        );
        world.get_manager_mut::<TransformComponent>().add(
            entity,
            TransformComponent::new(Transform::from_position(3.0, 0.0, 0.0)),
        );

        let entity = world.spawn_entity();
        world.get_manager_mut::<RenderableComponent>().add(
            entity,
            RenderableComponent::new(mesh3)
                .with_material(DefaultPipeline::main_pass(), material_1)
                .with_material(DefaultPipeline::outline_pass(), material_2),
        );
        world.get_manager_mut::<TransformComponent>().add(
            entity,
            TransformComponent::new(Transform::from_position(1.5, 0.0, 3.0)),
        );

        let camera_entity = world.spawn_entity();
        world
            .get_manager_mut::<CameraComponent>()
            .add(camera_entity, CameraComponent::new(true));
        world.get_manager_mut::<TransformComponent>().add(
            camera_entity,
            TransformComponent::new(Transform::from_position(1.5, 0.0, 6.0)),
        );

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
