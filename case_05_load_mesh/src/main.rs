use glotus::{
    AppConfig, Color, Entity, Material, Mesh, PointLight, Position, Shader, Transform, UniformValue
};

fn main() {
    let app = glotus::App::new_with_config(AppConfig {
        bg_color: [0.0, 0.0, 0.0],
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
        .set_position(Position::new(0.0, 0.0, 4.0));

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
