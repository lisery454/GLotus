use std::error::Error;

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new();

    app.borrow().build(|context| {
        let shader = context.borrow().with_sdr_mgr(|m| {
            m.create(ShaderConfig::new_vert_frag(
                ShaderInput::Source(include_str!("./assets/shaders/vs.vert").to_string()),
                ShaderInput::Source(include_str!("./assets/shaders/fs.frag").to_string()),
            ))
        })?;

        let material = context.borrow().with_mat_mgr(|m| m.create(shader))?;

        let mesh = context.borrow().with_msh_mgr(|m| {
            m.create_from_positions(
                vec![0, 1, 3, 1, 2, 3],
                vec![
                    1.0, 1.0, 0.0, // 0
                    1.0, -1.0, 0.0, // 1
                    -1.0, -1.0, 0.0, // 2
                    -1.0, 1.0, 0.0, // 3
                ],
            )
        })?;

        context.borrow().with_world(|w| {
            w.spawn_entity_with((
                Renderable::new(mesh).with_material(DefaultPipeline::main_pass(), material),
                Transform::default(),
            ));
            w.spawn_entity_with((Camera::new(true), Transform::from_position(0.0, 0.0, 10.0)));
        });

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
