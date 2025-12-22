use std::{collections::HashMap, error::Error};

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new();

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

        let material = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .material_manager
            .create(shader)?;

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
        let pass_name = DefaultPipeline::get_default_pass_name();

        let entity = world.spawn_entity();
        world.get_manager_mut::<RenderableComponent>().add(
            entity,
            RenderableComponent::new(HashMap::from([(pass_name.clone(), material)]), mesh),
        );
        world.get_manager_mut::<TransformComponent>().add(
            entity,
            TransformComponent::new(Transform::from_position(0.0, 0.0, 0.0)),
        );

        let entity = world.spawn_entity();
        world.get_manager_mut::<RenderableComponent>().add(
            entity,
            RenderableComponent::new(HashMap::from([(pass_name.clone(), material)]), mesh2),
        );
        world.get_manager_mut::<TransformComponent>().add(
            entity,
            TransformComponent::new(Transform::from_position(3.0, 0.0, 0.0)),
        );

        let entity = world.spawn_entity();
        world.get_manager_mut::<RenderableComponent>().add(
            entity,
            RenderableComponent::new(HashMap::from([(pass_name.clone(), material)]), mesh3),
        );
        world.get_manager_mut::<TransformComponent>().add(
            entity,
            TransformComponent::new(Transform::new(
                Translation::new(0.0, -1.2, 0.0),
                Default::default(),
                Scaling::new(100.0, 0.1, 100.0),
            )),
        );

        let camera_entity = world.spawn_entity();
        world
            .get_manager_mut::<CameraComponent>()
            .add(camera_entity, CameraComponent::new(true));
        world.get_manager_mut::<TransformComponent>().add(
            camera_entity,
            TransformComponent::new(Transform::from_position(1.5, 0.0, 6.0)),
        );

        let point_light_entity = world.spawn_entity();
        let mut point_light = PointLight::new();
        point_light.color = Color::from_rgb(255, 255, 255);
        point_light.intensity = 4.0;
        point_light.range = 20.0;
        world
            .get_manager_mut::<LightComponent>()
            .add(point_light_entity, LightComponent::new(point_light));
        world.get_manager_mut::<TransformComponent>().add(
            point_light_entity,
            TransformComponent::new(Transform::from_position(5.0, 6.0, 3.0)),
        );

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
