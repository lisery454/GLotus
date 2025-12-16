use glotus::*;

fn main() {
    let app = glotus::App::new();

    let shader = Shader::from_sources(
        include_str!("../assets/shaders/vs_0.vert"),
        include_str!("../assets/shaders/fs_0.frag"),
    )
    .unwrap();

    let material = Material::new(shader.clone());
    let pass_name = DefaultPipeline::get_default_pass_name();
    let material_group = MaterialGroup::single(pass_name, material.clone());

    let mesh = Mesh::from_position(
        &vec![0, 1, 3, 1, 2, 3],
        &vec![
            1.0, 1.0, -5.0, // 0
            1.0, -1.0, -5.0, // 1
            -1.0, -1.0, -5.0, // 2
            -1.0, 1.0, -5.0, // 3
        ],
    );

    let entity = Entity::new(Transform::default(), material_group.clone(), mesh.clone());

    app.borrow()
        .get_world()
        .borrow_mut()
        .add_entity(entity.clone());

    app.borrow_mut().init_camera_tickable();

    app.borrow_mut().run();
}
