use std::error::Error;

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new_with_config(AppConfig {
        anti_pixel: AntiPixel::MSAA16,
        ..Default::default()
    });

    app.borrow().build(|context| {
        let shader = context.borrow().create_shader(ShaderConfig::new_vert_frag(
            ShaderInput::Source(include_str!("./assets/shaders/vs.vert").to_string()),
            ShaderInput::Source(include_str!("./assets/shaders/fs.frag").to_string()),
        ))?;

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
            Renderable::new(mesh).with_material(DefaultPipeline::main_pass(), material),
            Transform::from_position(0.0, 0.0, 0.0),
        ));

        context.borrow().spawn_entity_with((
            Renderable::new(mesh2).with_material(DefaultPipeline::main_pass(), material),
            Transform::from_position(3.0, 0.0, 0.0),
        ));

        context.borrow().spawn_entity_with((
            Renderable::new(mesh3).with_material(DefaultPipeline::main_pass(), material),
            Transform::from_position(0.0, 0.0, 3.0),
        ));

        context.borrow().spawn_entity_with((
            Renderable::new(mesh4).with_material(DefaultPipeline::main_pass(), material),
            Transform::from_position(3.0, 0.0, 3.0),
        ));

        context
            .borrow()
            .spawn_entity_with((Transform::from_position(1.5, 0.0, 6.0), Camera::new(true)));

        context.borrow().spawn_entity_with((
            Light::point()
                .with_color(Color::WHITE)
                .with_intensity(4.0)
                .with_range(20.0),
            Transform::from_position(5.0, 6.0, 3.0),
        ));

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
