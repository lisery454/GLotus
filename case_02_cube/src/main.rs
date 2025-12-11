use glotus::*;

fn main() {
    let app = glotus::App::new();

    let shader = Shader::from_files(
        concat!(env!("CARGO_PKG_NAME"), "/assets/shaders/vs.vert"),
        concat!(env!("CARGO_PKG_NAME"), "/assets/shaders/fs.frag"),
    )
    .unwrap();

    let texture = Texture2D::from_file_default(concat!(
        env!("CARGO_PKG_NAME"),
        "/assets/textures/brick.png"
    ))
    .unwrap();

    let material = Material::new(shader.clone());

    material
        .borrow_mut()
        .insert_uniform("texture1", UniformValue::Texture(0, texture.clone()));

    let mesh = Mesh::from_position_texcoord(
        &vec![
            -0.5, -0.5, -0.5, // - - - 0
            0.5, -0.5, -0.5, // + - - 1
            0.5, 0.5, -0.5, // + + - 2
            -0.5, 0.5, -0.5, // - + - 3
            -0.5, -0.5, 0.5, // - - + 4
            0.5, -0.5, 0.5, // + - + 5
            0.5, 0.5, 0.5, // + + + 6
            -0.5, 0.5, 0.5, // - + + 7
        ],
        &vec![
            0, 1, 2, 2, 3, 0, // back
            4, 5, 6, 6, 7, 4, // front
            7, 3, 0, 0, 4, 7, // left
            1, 2, 6, 6, 5, 1, // right
            2, 3, 7, 7, 6, 2, // top
            1, 2, 4, 4, 5, 1, // bottom
        ],
        &vec![
            0.0, 0.0, // 0
            1.0, 0.0, // 1
            1.0, 1.0, // 2
            0.0, 1.0, // 3
        ],
        &vec![
            0, 1, 2, 2, 3, 0, // back
            0, 1, 2, 2, 3, 0, // front
            0, 1, 2, 2, 3, 0, // left
            0, 1, 2, 2, 3, 0, // right
            0, 1, 2, 2, 3, 0, // top
            0, 1, 2, 2, 3, 0, // bottom
        ],
    );

    let entity = Entity::new(Transform::default(), material.clone(), mesh.clone());

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
