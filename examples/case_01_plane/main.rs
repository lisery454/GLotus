use std::error::Error;

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new();

    app.borrow().build(|context| {
        let shader = context.borrow().create_shader_from_sources(
            include_str!("./assets/shaders/vs.vert"),
            include_str!("./assets/shaders/fs.frag"),
        )?;

        let material = context.borrow().create_material(shader)?;

        let mesh = context.borrow().create_mesh_from_position(
            &vec![0, 1, 3, 1, 2, 3],
            &vec![
                1.0, 1.0, -5.0, // 0
                1.0, -1.0, -5.0, // 1
                -1.0, -1.0, -5.0, // 2
                -1.0, 1.0, -5.0, // 3
            ],
        )?;

        let entity = context.borrow().spawn_entity();
        context.borrow().add_component(
            entity,
            RenderableComponent::new(mesh).with_material(DefaultPipeline::main_pass(), material),
        );
        context
            .borrow()
            .add_component(entity, TransformComponent::new(Transform::default()));
        let camera_entity = context.borrow().spawn_entity();
        context
            .borrow()
            .add_component(camera_entity, CameraComponent::new(true));
        context.borrow().add_component(
            camera_entity,
            TransformComponent::new(Transform::from_position(0.0, 0.0, 10.0)),
        );

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
