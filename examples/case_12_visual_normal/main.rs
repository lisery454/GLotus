use std::error::Error;

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = glotus::App::new_with_config(AppConfig {
        ..Default::default()
    });

    app.borrow().build(|context| {
        let solid_shader = context.borrow().create_shader_from_sources_vf(
            include_str!("./assets/shaders/solid.vert"),
            include_str!("./assets/shaders/solid.frag"),
        )?;

        let solid_material = context.borrow().get_material_builder(solid_shader)?.build();

        let normal_shader = context.borrow().create_shader_from_sources_vfg(
            include_str!("./assets/shaders/normal.vert"),
            include_str!("./assets/shaders/normal.frag"),
            include_str!("./assets/shaders/normal.geom"),
        )?;
        let normal_material = context.borrow().create_material(normal_shader)?;

        let ball_mesh = context
            .borrow()
            .create_mesh_from_obj_in_bytes(include_bytes!("./assets/meshes/ball.obj"))?;

        context.borrow().spawn_entity_with((
            Renderable::new(ball_mesh)
                .with_material(DefaultPipeline::main_pass(), solid_material)
                .with_material(DefaultPipeline::debug_pass(), normal_material),
            Transform::default(),
        ));

        context
            .borrow()
            .spawn_entity_with((Camera::new(true), Transform::from_position(0.0, 1.0, 4.0)));

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
