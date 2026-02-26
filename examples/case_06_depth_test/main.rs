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
            m.create_from_obj_bytes(include_bytes!("./assets/meshes/sphere.obj"))
        })?;

        let mesh2 = context.borrow().with_msh_mgr(|m| {
            m.create_from_obj_bytes(include_bytes!("./assets/meshes/sphere_no_smooth.obj"))
        })?;

        let mesh3 = context
            .borrow()
            .with_msh_mgr(|m| m.create_from_obj_bytes(include_bytes!("./assets/meshes/box.obj")))?;

        context.borrow().with_world(|w| {
            w.spawn_entity_with((
                Transform::from_position(0.0, 0.0, 0.0),
                Renderable::new(mesh).with_material(DefaultPipeline::main_pass(), material),
            ));

            w.spawn_entity_with((
                Renderable::new(mesh2).with_material(DefaultPipeline::main_pass(), material),
                Transform::from_position(3.0, 0.0, 0.0),
            ));

            w.spawn_entity_with((
                Renderable::new(mesh3).with_material(DefaultPipeline::main_pass(), material),
                Transform::new(
                    Translation::new(0.0, -1.2, 0.0),
                    Default::default(),
                    Scaling::new(100.0, 0.1, 100.0),
                ),
            ));

            w.spawn_entity_with((Camera::new(true), Transform::from_position(1.5, 0.0, 6.0)));
        });

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
