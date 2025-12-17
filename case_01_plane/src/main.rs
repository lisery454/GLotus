use std::error::Error;

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = glotus::App::new();

    let shader_handle = app
        .borrow()
        .get_shader_manager()
        .borrow()
        .create_from_sources(
            include_str!("../assets/shaders/vs_0.vert"),
            include_str!("../assets/shaders/fs_0.frag"),
        )?;

    let material_handle = app
        .borrow()
        .get_material_manager()
        .borrow()
        .create(shader_handle)?;

    let pass_name = DefaultPipeline::get_default_pass_name();

    let mesh_handle = app
        .borrow()
        .get_mesh_manager()
        .borrow()
        .create_from_position(
            &vec![0, 1, 3, 1, 2, 3],
            &vec![
                1.0, 1.0, -5.0, // 0
                1.0, -1.0, -5.0, // 1
                -1.0, -1.0, -5.0, // 2
                -1.0, 1.0, -5.0, // 3
            ],
        )?;

    let entity_handle = app
        .borrow()
        .get_entity_manager()
        .borrow()
        .create(HashMap::from([(pass_name, material_handle)]), mesh_handle);

    app.borrow()
        .get_world()
        .borrow_mut()
        .add_entity(entity.clone());

    app.borrow_mut().init_camera_tickable();

    app.borrow_mut().run();

    Ok(())
}
