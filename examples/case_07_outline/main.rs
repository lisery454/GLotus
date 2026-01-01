use std::error::Error;

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new_with_config(AppConfig {
        anti_pixel: AntiPixel::MSAA16,
        ..Default::default()
    });

    app.borrow().build(|context| {
        let shader_1 = context.borrow().create_shader_from_sources(
            include_str!("./assets/shaders/vs_1.vert"),
            include_str!("./assets/shaders/fs_1.frag"),
        )?;

        let shader_2 = context.borrow().create_shader_from_sources(
            include_str!("./assets/shaders/vs_2.vert"),
            include_str!("./assets/shaders/fs_2.frag"),
        )?;

        let material_1 = context.borrow().create_material(shader_1)?;
        let material_2 = context.borrow().create_material(shader_2)?;

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

        context.borrow().spawn_entity_with((
            Renderable::new(mesh)
                .with_material(DefaultPipeline::main_pass(), material_1)
                .with_material(DefaultPipeline::outline_pass(), material_2),
            Transform::from_position(0.0, 0.0, 0.0),
        ));

        context.borrow().spawn_entity_with((
            Renderable::new(mesh2)
                .with_material(DefaultPipeline::main_pass(), material_1)
                .with_material(DefaultPipeline::outline_pass(), material_2),
            Transform::from_position(3.0, 0.0, 0.0),
        ));

        context.borrow().spawn_entity_with((
            Renderable::new(mesh3)
                .with_material(DefaultPipeline::main_pass(), material_1)
                .with_material(DefaultPipeline::outline_pass(), material_2),
            Transform::from_position(1.5, 0.0, 3.0),
        ));

        context
            .borrow()
            .spawn_entity_with((Transform::from_position(1.5, 0.0, 6.0), Camera::new(true)));

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
