use std::error::Error;

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new_with_config(AppConfig {
        anti_pixel_msaa: AntiPixel::MSAA16,
        ..Default::default()
    });

    app.borrow().build(|context| {
        let shader_1 = context.borrow().create_shader_from_sources(
            include_str!("./assets/shaders/vs_1.vert"),
            include_str!("./assets/shaders/fs_1.frag"),
        )?;

        let shader_2 = context.borrow().create_shader_from_sources(
            include_str!("./assets/shaders/vs_2.vert"),
            include_str!("./assets/shaders/fs_2.frag"),
        )?;

        let material_1 = context.borrow().create_material(shader_1)?;
        let material_2 = context.borrow().create_material(shader_2)?;

        let mesh = context
            .borrow()
            .create_mesh_from_obj_in_bytes(include_bytes!("./assets/meshes/sphere.obj"))?;

        let mesh2 = context
            .borrow()
            .create_mesh_from_obj_in_bytes(include_bytes!(
                "./assets/meshes/sphere_no_smooth.obj"
            ))?;

        let mesh3 = context
            .borrow()
            .create_mesh_from_obj_in_bytes(include_bytes!("./assets/meshes/box.obj"))?;

        let entity = context.borrow().spawn_entity();
        context.borrow().add_component(
            entity,
            RenderableComponent::new(mesh)
                .with_material(DefaultPipeline::main_pass(), material_1)
                .with_material(DefaultPipeline::outline_pass(), material_2),
        );
        context.borrow().add_component(
            entity,
            TransformComponent::new(Transform::from_position(0.0, 0.0, 0.0)),
        );

        let entity = context.borrow().spawn_entity();
        context.borrow().add_component(
            entity,
            RenderableComponent::new(mesh2)
                .with_material(DefaultPipeline::main_pass(), material_1)
                .with_material(DefaultPipeline::outline_pass(), material_2),
        );
        context.borrow().add_component(
            entity,
            TransformComponent::new(Transform::from_position(3.0, 0.0, 0.0)),
        );

        let entity = context.borrow().spawn_entity();
        context.borrow().add_component(
            entity,
            RenderableComponent::new(mesh3)
                .with_material(DefaultPipeline::main_pass(), material_1)
                .with_material(DefaultPipeline::outline_pass(), material_2),
        );
        context.borrow().add_component(
            entity,
            TransformComponent::new(Transform::from_position(1.5, 0.0, 3.0)),
        );

        let camera_entity = context.borrow().spawn_entity();
        context
            .borrow()
            .add_component(camera_entity, CameraComponent::new(true));
        context.borrow().add_component(
            camera_entity,
            TransformComponent::new(Transform::from_position(1.5, 0.0, 6.0)),
        );

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
