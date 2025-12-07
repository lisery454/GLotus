use glotus::{
    AppConfig, Color, DirectionalLight, Entity, FilteringMode, Material, Mesh, PointLight,
    Position, Rotation, Shader, SpotLight, Texture2D, Transform, UniformValue, Vertex,
    WrappingMode,
};

fn main() {
    let app = glotus::App::new_with_config(AppConfig {
        bg_color: [0.0, 0.0, 0.0],
        ..Default::default()
    });

    app.borrow_mut().init_camera_tickable();

    let shader = Shader::from_files(
        concat!(env!("CARGO_PKG_NAME"), "/assets/shaders/vs.vert"),
        concat!(env!("CARGO_PKG_NAME"), "/assets/shaders/fs.frag"),
    )
    .unwrap();

    let texture_diffuse = Texture2D::from_file(
        concat!(
            env!("CARGO_PKG_NAME"),
            "/assets/textures/texture_diffuse.png"
        ),
        WrappingMode::Repeat,
        WrappingMode::Repeat,
        FilteringMode::LinearMipmapLinear,
        FilteringMode::Linear,
    )
    .unwrap();

    let texture_specular = Texture2D::from_file(
        concat!(
            env!("CARGO_PKG_NAME"),
            "/assets/textures/texture_specular.png"
        ),
        WrappingMode::Repeat,
        WrappingMode::Repeat,
        FilteringMode::LinearMipmapLinear,
        FilteringMode::Linear,
    )
    .unwrap();

    let material = Material::new(shader.clone());
    material
        .borrow_mut()
        .insert_uniform("material.diffuse_texture", UniformValue::Texture(0));
    material
        .borrow_mut()
        .insert_uniform("material.specular_texture", UniformValue::Texture(1));
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

    material
        .borrow_mut()
        .insert_textures(0, texture_diffuse.clone());
    material
        .borrow_mut()
        .insert_textures(1, texture_specular.clone());

    let mesh = Mesh::new(
        vec![
            // back
            Vertex::from_position_and_normal_and_tex_coords(
                -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 0.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(
                0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 0.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(
                0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 1.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(
                0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 1.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(
                -0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 1.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(
                -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 0.0,
            ),
            // front
            Vertex::from_position_and_normal_and_tex_coords(
                -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(
                0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 0.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 1.0),
            Vertex::from_position_and_normal_and_tex_coords(0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 1.0),
            Vertex::from_position_and_normal_and_tex_coords(
                -0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 1.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(
                -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0,
            ),
            // left
            Vertex::from_position_and_normal_and_tex_coords(
                -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, 1.0, 0.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(
                -0.5, 0.5, -0.5, -1.0, 0.0, 0.0, 1.0, 1.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(
                -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, 0.0, 1.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(
                -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, 0.0, 1.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(
                -0.5, -0.5, 0.5, -1.0, 0.0, 0.0, 0.0, 0.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(
                -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, 1.0, 0.0,
            ),
            // right
            Vertex::from_position_and_normal_and_tex_coords(0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0),
            Vertex::from_position_and_normal_and_tex_coords(
                0.5, 0.5, -0.5, 1.0, 0.0, 0.0, 1.0, 1.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(
                0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 1.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(
                0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 1.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(
                0.5, -0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 0.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0),
            // down
            Vertex::from_position_and_normal_and_tex_coords(
                -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 0.0, 1.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(
                0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 1.0, 1.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(
                0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 1.0, 0.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(
                0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 1.0, 0.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(
                -0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 0.0, 0.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(
                -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 0.0, 1.0,
            ),
            // up
            Vertex::from_position_and_normal_and_tex_coords(
                -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 1.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(
                0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 1.0, 1.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0),
            Vertex::from_position_and_normal_and_tex_coords(0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0),
            Vertex::from_position_and_normal_and_tex_coords(
                -0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 0.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(
                -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 1.0,
            ),
        ],
        (0..=35).collect(),
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
        .set_position(Position::new(0.0, 1.0, 4.0));

    let point_light = PointLight::new();
    point_light.borrow_mut().color = Color::from_rgb(0, 255, 0);
    point_light.borrow_mut().intensity = 3.0;
    point_light.borrow_mut().range = 10.0;
    point_light
        .borrow_mut()
        .transform
        .set_position(Position::new(0.0, 0.0, 0.0));
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
        .set_position(Position::new(0.0, 0.0, 8.0));
    spot_light
        .borrow_mut()
        .transform
        .set_rotation(Rotation::new(0.0, 0.0, 0.0));
    app.borrow().get_world().borrow_mut().add_light(spot_light);

    app.borrow_mut().run();
}
