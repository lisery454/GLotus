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
            RenderableComponent::new(mesh).with_material(DefaultPipeline::main_pass(), material),
        );
        context.borrow().add_component(
            entity,
            TransformComponent::new(Transform::from_position(0.0, 0.0, 0.0)),
        );

        let entity = context.borrow().spawn_entity();
        context.borrow().add_component(
            entity,
            RenderableComponent::new(mesh2).with_material(DefaultPipeline::main_pass(), material),
        );
        context.borrow().add_component(
            entity,
            TransformComponent::new(Transform::from_position(3.0, 0.0, 0.0)),
        );

        let entity = context.borrow().spawn_entity();
        context.borrow().add_component(
            entity,
            RenderableComponent::new(mesh3).with_material(DefaultPipeline::main_pass(), material),
        );
        context.borrow().add_component(
            entity,
            TransformComponent::new(Transform::new(
                Translation::new(0.0, -1.2, 0.0),
                Default::default(),
                Scaling::new(100.0, 0.1, 100.0),
            )),
        );

        let camera_entity = context.borrow().spawn_entity();
        context
            .borrow()
            .add_component(camera_entity, CameraComponent::new(true));
        context.borrow().add_component(
            camera_entity,
            TransformComponent::new(Transform::from_position(1.5, 0.0, 6.0)),
        );

        let point_light_entity = context.borrow().spawn_entity();
        let mut point_light = PointLight::new();
        point_light.color = Color::from_rgb(255, 255, 255);
        point_light.intensity = 4.0;
        point_light.range = 20.0;
        context
            .borrow()
            .add_component(point_light_entity, LightComponent::new(point_light));
        context.borrow().add_component(
            point_light_entity,
            TransformComponent::new(Transform::from_position(5.0, 6.0, 3.0)),
        );

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
