use glotus::*;

fn main() {
    let app = glotus::App::new();

    let shader = Shader::from_sources(
        include_str!("../assets/shaders/vs.vert"),
        include_str!("../assets/shaders/fs.frag"),
    )
    .unwrap();

    let texture =
        Texture2D::from_byte_default(include_bytes!("../assets/textures/brick.png")).unwrap();

    let material = Material::new(shader.clone());
    let pass_name = get_default_pipeline_default_pass_name();
    let material_group = MaterialGroup::single(pass_name, material.clone());

    material
        .borrow_mut()
        .insert_uniform("texture1", UniformValue::Texture(0, texture.clone()));

    let mesh = Mesh::from_position_texcoord(
        &(0..36).collect(),
        &vec![
            // back
            -0.5, -0.5, -0.5, // - - - 0
            0.5, -0.5, -0.5, // + - - 1
            0.5, 0.5, -0.5, // + + - 2
            0.5, 0.5, -0.5, // + + - 2
            -0.5, 0.5, -0.5, // - + - 3
            -0.5, -0.5, -0.5, // - - - 0
            // front
            -0.5, -0.5, 0.5, // - - + 4
            0.5, -0.5, 0.5, // + - + 5
            0.5, 0.5, 0.5, // + + + 6
            0.5, 0.5, 0.5, // + + + 6
            -0.5, 0.5, 0.5, // - + + 7
            -0.5, -0.5, 0.5, // - - + 4
            // left
            -0.5, 0.5, 0.5, // - + + 7
            -0.5, 0.5, -0.5, // - + - 3
            -0.5, -0.5, -0.5, // - - - 0
            -0.5, -0.5, -0.5, // - - - 0
            -0.5, -0.5, 0.5, // - - + 4
            -0.5, 0.5, 0.5, // - + + 7
            // right
            0.5, -0.5, -0.5, // + - - 1
            0.5, 0.5, -0.5, // + + - 2
            0.5, 0.5, 0.5, // + + + 6
            0.5, 0.5, 0.5, // + + + 6
            0.5, -0.5, 0.5, // + - + 5
            0.5, -0.5, -0.5, // + - - 1
            // top
            0.5, 0.5, -0.5, // + + - 2
            -0.5, 0.5, -0.5, // - + - 3
            -0.5, 0.5, 0.5, // - + + 7
            -0.5, 0.5, 0.5, // - + + 7
            0.5, 0.5, 0.5, // + + + 6
            0.5, 0.5, -0.5, // + + - 2
            // bottom
            0.5, -0.5, -0.5, // + - - 1
            -0.5, -0.5, -0.5, // - - - 0
            -0.5, -0.5, 0.5, // - - + 4
            -0.5, -0.5, 0.5, // - - + 4
            0.5, -0.5, 0.5, // + - + 5
            0.5, -0.5, -0.5, // + - - 1
        ],
        &vec![
            0.0, 0.0, // 0
            1.0, 0.0, // 1
            1.0, 1.0, // 2
            1.0, 1.0, // 2
            0.0, 1.0, // 3
            0.0, 0.0, // 0
        ]
        .repeat(6),
    );

    let entity = Entity::new(Transform::default(), material_group.clone(), mesh.clone());

    app.borrow()
        .get_world()
        .borrow_mut()
        .add_entity(entity.clone());

    app.borrow()
        .get_world()
        .borrow()
        .get_camera()
        .borrow_mut()
        .get_transform_mut()
        .set_translation(Translation::new(0.0, 0.0, 10.0));

    app.borrow_mut().init_camera_tickable();

    app.borrow_mut().run();
}
