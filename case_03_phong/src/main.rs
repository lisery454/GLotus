use glotus::*;

fn main() {
    let app = glotus::App::new_with_config(AppConfig {
        bg_color: [0.0, 0.0, 0.0],
        ..Default::default()
    });

    app.borrow_mut().init_camera_tickable();

    let shader = Shader::from_sources(
        include_str!("../assets/shaders/vs.vert"),
        include_str!("../assets/shaders/fs.frag"),
    )
    .unwrap();

    let texture_diffuse =
        Texture2D::from_byte_default(include_bytes!("../assets/textures/texture_diffuse.png"))
            .unwrap();

    let texture_specular =
        Texture2D::from_byte_default(include_bytes!("../assets/textures/texture_specular.png"))
            .unwrap();

    let material = Material::new(shader.clone());
    material.borrow_mut().insert_uniform(
        "material.diffuse_texture",
        UniformValue::Texture(0, texture_diffuse.clone()),
    );
    material.borrow_mut().insert_uniform(
        "material.specular_texture",
        UniformValue::Texture(1, texture_specular.clone()),
    );
    material.borrow_mut().insert_uniform(
        "material.ambient_factor",
        UniformValue::Vector3([0.2, 0.2, 0.2]),
    );
    material.borrow_mut().insert_uniform(
        "material.diffuse_factor",
        UniformValue::Vector3([1.0, 1.0, 1.0]),
    );
    material.borrow_mut().insert_uniform(
        "material.specular_factor",
        UniformValue::Vector3([0.8, 0.8, 0.8]),
    );
    material
        .borrow_mut()
        .insert_uniform("material.specular_shininess", UniformValue::Float(256.0));

    let mesh = Mesh::from_position_normal_texcoord(
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
            1, 0, 4, 4, 5, 1, // bottom
        ],
        &vec![
            0.0, 0.0, -1.0, // back
            0.0, 0.0, 1.0, // front
            -1.0, 0.0, 0.0, // left
            1.0, 0.0, 0.0, // right
            0.0, 1.0, 0.0, // top
            0.0, -1.0, 0.0, // bottom
        ],
        &vec![
            0, 0, 0, 0, 0, 0, // back
            1, 1, 1, 1, 1, 1, // front
            2, 2, 2, 2, 2, 2, // left
            3, 3, 3, 3, 3, 3, //right
            4, 4, 4, 4, 4, 4, //top
            5, 5, 5, 5, 5, 5, //bottom
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

    for i in -1..2 {
        for j in -1..2 {
            for k in -1..2 {
                if i == 0 && j == 0 && k == 0 {
                    continue;
                }
                let entity = Entity::new(
                    Transform::from_position(3.0 * (i as f32), 3.0 * (j as f32), 3.0 * (k as f32)),
                    material.clone(),
                    mesh.clone(),
                );

                app.borrow()
                    .get_world()
                    .borrow_mut()
                    .add_entity(entity.clone());
            }
        }
    }

    app.borrow()
        .get_world()
        .borrow()
        .get_camera()
        .borrow_mut()
        .get_transform_mut()
        .set_translation(Translation::new(0.0, 1.0, 4.0));

    let point_light = PointLight::new();
    point_light.borrow_mut().color = Color::from_rgb(0, 255, 0);
    point_light.borrow_mut().intensity = 3.0;
    point_light.borrow_mut().range = 10.0;
    point_light
        .borrow_mut()
        .transform
        .set_translation(Translation::new(0.0, 0.0, 0.0));
    app.borrow().get_world().borrow_mut().add_light(point_light);

    let directional_light = DirectionalLight::new();
    directional_light.borrow_mut().color = Color::from_rgb(255, 0, 0);
    directional_light
        .borrow_mut()
        .transform
        .set_rotation(Rotation::new(0.0, 180.0, 0.0));
    app.borrow()
        .get_world()
        .borrow_mut()
        .add_light(directional_light);

    let spot_light = SpotLight::new();
    spot_light.borrow_mut().color = Color::from_rgb(0, 0, 255);
    spot_light
        .borrow_mut()
        .transform
        .set_translation(Translation::new(0.0, 0.0, 8.0));
    spot_light
        .borrow_mut()
        .transform
        .set_rotation(Rotation::new(0.0, 0.0, 0.0));
    app.borrow().get_world().borrow_mut().add_light(spot_light);

    app.borrow_mut().run();
}
