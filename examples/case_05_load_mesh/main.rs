use std::error::Error;

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new();

    app.borrow().build(|context| {
        let shader = context.borrow().create_shader_from_sources(
            include_str!("./assets/shaders/vs.vert"),
            include_str!("./assets/shaders/fs.frag"),
        )?;

        let material = context
            .borrow()
            .get_material_builder(shader)?
            .with(
                "material.diff_color",
                UniformValue::Vector3([0.5, 0.5, 0.5]),
            )
            .with(
                "material.spec_color",
                UniformValue::Vector3([1.0, 1.0, 1.0]),
            )
            .with(
                "material.ambient_factor",
                UniformValue::Vector3([0.1, 0.1, 0.1]),
            )
            .with(
                "material.diffuse_factor",
                UniformValue::Vector3([1.0, 1.0, 1.0]),
            )
            .with(
                "material.specular_factor",
                UniformValue::Vector3([0.6, 0.6, 0.6]),
            )
            .with("material.specular_shininess", UniformValue::Float(40.0))
            .build();

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

        let mesh4 = context
            .borrow()
            .create_mesh_from_obj_in_bytes(include_bytes!("./assets/meshes/suzanne.obj"))?;

        context.borrow().spawn_entity_with((
            RenderableComponent::new(mesh).with_material(DefaultPipeline::main_pass(), material),
            TransformComponent::new(Transform::from_position(0.0, 0.0, 0.0)),
        ));

        context.borrow().spawn_entity_with((
            RenderableComponent::new(mesh2).with_material(DefaultPipeline::main_pass(), material),
            TransformComponent::new(Transform::from_position(3.0, 0.0, 0.0)),
        ));

        context.borrow().spawn_entity_with((
            RenderableComponent::new(mesh3).with_material(DefaultPipeline::main_pass(), material),
            TransformComponent::new(Transform::from_position(0.0, 0.0, 3.0)),
        ));

        context.borrow().spawn_entity_with((
            RenderableComponent::new(mesh4).with_material(DefaultPipeline::main_pass(), material),
            TransformComponent::new(Transform::from_position(3.0, 0.0, 3.0)),
        ));

        context.borrow().spawn_entity_with((
            TransformComponent::new(Transform::from_position(1.5, 0.0, 6.0)),
            CameraComponent::new(true),
        ));

        let mut point_light = PointLight::new();
        point_light.color = Color::from_rgb(255, 255, 255);
        point_light.intensity = 4.0;
        point_light.range = 20.0;
        context.borrow().spawn_entity_with((
            LightComponent::new(point_light),
            TransformComponent::new(Transform::from_position(5.0, 6.0, 3.0)),
        ));

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
