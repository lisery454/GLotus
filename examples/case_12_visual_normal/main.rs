use std::error::Error;

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = glotus::App::new_with_config(AppConfig {
        ..Default::default()
    });

    app.borrow().build(|context| {
        let solid_shader = context.borrow().with_sdr_mgr(|m| {
            m.create(ShaderConfig::new_vert_frag(
                ShaderInput::Source(include_str!("./assets/shaders/solid.vert").to_string()),
                ShaderInput::Source(include_str!("./assets/shaders/solid.frag").to_string()),
            ))
        })?;

        let solid_material = context
            .borrow()
            .with_mat_mgr(|m| m.get_builder(solid_shader)?.build())?;

        let normal_shader = context.borrow().with_sdr_mgr(|m| {
            m.create(ShaderConfig::new_vert_frag_gemo(
                ShaderInput::Source(include_str!("./assets/shaders/normal.vert").to_string()),
                ShaderInput::Source(include_str!("./assets/shaders/normal.frag").to_string()),
                ShaderInput::Source(include_str!("./assets/shaders/normal.geom").to_string()),
            ))
        })?;
        let normal_material = context
            .borrow()
            .with_mat_mgr(|m| m.get_builder(normal_shader)?.build())?;

        let ball_mesh = context.borrow().with_msh_mgr(|m| {
            m.create_from_obj_bytes(include_bytes!("./assets/meshes/ball.obj"))
        })?;

        context.borrow().with_world(|w| {
            w.spawn_entity_with((
                Renderable::new(ball_mesh)
                    .with_material(DefaultPipeline::main_pass(), solid_material)
                    .with_material(DefaultPipeline::debug_pass(), normal_material),
                Transform::default(),
            ));

            w.spawn_entity_with((Camera::new(true), Transform::from_position(0.0, 1.0, 4.0)));
        });

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
