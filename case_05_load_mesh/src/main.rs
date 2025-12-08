use glotus::{
    AntiPixel, AppConfig, Color, Entity, Material, Mesh, PointLight, Position, Shader, Transform,
    UniformValue,
};

fn main() {
    let app = glotus::App::new_with_config(AppConfig {
        // bg_color: [0.0, 0.0, 0.0],
        anti_pixel_msaa: AntiPixel::MSAA16,
        ..Default::default()
    });

    let shader = Shader::from_files(
        concat!(env!("CARGO_PKG_NAME"), "/assets/shaders/vs_0.vert"),
        concat!(env!("CARGO_PKG_NAME"), "/assets/shaders/fs_0.frag"),
    )
    .unwrap();

    let material = Material::new(shader.clone());
    material.borrow_mut().insert_uniform(
        "material.diff_color",
        UniformValue::Vector3([0.5, 0.5, 0.5]),
    );
    material.borrow_mut().insert_uniform(
        "material.spec_color",
        UniformValue::Vector3([1.0, 1.0, 1.0]),
    );
    material.borrow_mut().insert_uniform(
        "material.ambient_factor",
        UniformValue::Vector3([0.1, 0.1, 0.1]),
    );
    material.borrow_mut().insert_uniform(
        "material.diffuse_factor",
        UniformValue::Vector3([1.0, 1.0, 1.0]),
    );
    material.borrow_mut().insert_uniform(
        "material.specular_factor",
        UniformValue::Vector3([0.6, 0.6, 0.6]),
    );
    material
        .borrow_mut()
        .insert_uniform("material.specular_shininess", UniformValue::Float(40.0));

    let mesh = Mesh::load_obj(concat!(env!("CARGO_PKG_NAME"), "/assets/meshes/sphere.obj"));
    let mesh_2 = Mesh::load_obj(concat!(
        env!("CARGO_PKG_NAME"),
        "/assets/meshes/sphere_no_smooth.obj"
    ));
    let mesh_3 = Mesh::load_obj(concat!(env!("CARGO_PKG_NAME"), "/assets/meshes/box.obj"));
    let mesh_4 = Mesh::load_obj(concat!(
        env!("CARGO_PKG_NAME"),
        "/assets/meshes/suzanne.obj"
    ));

    app.borrow()
        .get_world()
        .borrow_mut()
        .add_entity(Entity::new(
            Transform::from_position(0.0, 0.0, 0.0),
            material.clone(),
            mesh.clone(),
        ));

    app.borrow()
        .get_world()
        .borrow_mut()
        .add_entity(Entity::new(
            Transform::from_position(3.0, 0.0, 0.0),
            material.clone(),
            mesh_2.clone(),
        ));

    app.borrow()
        .get_world()
        .borrow_mut()
        .add_entity(Entity::new(
            Transform::from_position(0.0, 0.0, 3.0),
            material.clone(),
            mesh_3.clone(),
        ));

    app.borrow()
        .get_world()
        .borrow_mut()
        .add_entity(Entity::new(
            Transform::from_position(3.0, 0.0, 3.0),
            material.clone(),
            mesh_4.clone(),
        ));

    app.borrow()
        .get_world()
        .borrow()
        .get_camera()
        .borrow_mut()
        .get_transform_mut()
        .set_position(Position::new(1.5, 0.0, 6.0));

    let point_light = PointLight::new();
    point_light.borrow_mut().color = Color::from_rgb(255, 255, 255);
    point_light.borrow_mut().intensity = 4.0;
    point_light.borrow_mut().range = 20.0;
    point_light
        .borrow_mut()
        .transform
        .set_position(Position::new(5.0, 6.0, 3.0));
    app.borrow()
        .get_world()
        .borrow_mut()
        .add_light(point_light.clone());

    app.borrow_mut().init_camera_tickable();

    app.borrow_mut().run();
}
