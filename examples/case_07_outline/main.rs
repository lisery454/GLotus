use std::error::Error;

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new_with_config(AppConfig {
        anti_pixel: AntiPixel::MSAA16,
        ..Default::default()
    });

    app.borrow().build(|context| {
        let shader_1 = context.borrow().with_sdr_mgr(|m| {
            m.create(ShaderConfig::new_vert_frag(
                ShaderInput::Source(include_str!("./assets/shaders/normal.vert").to_string()),
                ShaderInput::Source(include_str!("./assets/shaders/normal.frag").to_string()),
            ))
        })?;

        let shader_2 = context.borrow().with_sdr_mgr(|m| {
            m.create(ShaderConfig::new_vert_frag(
                ShaderInput::Source(include_str!("./assets/shaders/outline.vert").to_string()),
                ShaderInput::Source(include_str!("./assets/shaders/outline.frag").to_string()),
            ))
        })?;

        let material_1 = context.borrow().with_mat_mgr(|m| m.create(shader_1))?;
        let material_2 = context.borrow().with_mat_mgr(|m| m.create(shader_2))?;

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
                Renderable::new(mesh)
                    .with_material(DefaultPipeline::main_pass(), material_1)
                    .with_material(DefaultPipeline::outline_pass(), material_2),
                Transform::from_position(0.0, 0.0, 0.0),
            ));

            w.spawn_entity_with((
                Renderable::new(mesh2)
                    .with_material(DefaultPipeline::main_pass(), material_1)
                    .with_material(DefaultPipeline::outline_pass(), material_2),
                Transform::from_position(3.0, 0.0, 0.0),
            ));

            w.spawn_entity_with((
                Renderable::new(mesh3)
                    .with_material(DefaultPipeline::main_pass(), material_1)
                    .with_material(DefaultPipeline::outline_pass(), material_2),
                Transform::from_position(1.5, 0.0, 3.0),
            ));

            w.spawn_entity_with((Transform::from_position(1.5, 0.0, 6.0), Camera::new(true)));
        });

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
