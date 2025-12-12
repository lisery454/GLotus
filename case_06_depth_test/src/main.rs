use glotus::*;

fn main() {
    let app = glotus::App::new_with_config(AppConfig {
        // bg_color: [0.0, 0.0, 0.0],
        anti_pixel_msaa: AntiPixel::MSAA16,
        ..Default::default()
    });

    let shader = Shader::from_sources(
        include_str!("../assets/shaders/vs_0.vert"),
        include_str!("../assets/shaders/fs_0.frag"),
    )
    .unwrap();

    let material = Material::new(shader.clone());

    let mesh = Mesh::load_obj_from_memory(include_bytes!("../assets/meshes/sphere.obj")).unwrap();
    let mesh_2 =
        Mesh::load_obj_from_memory(include_bytes!("../assets/meshes/sphere_no_smooth.obj"))
            .unwrap();
    let mesh_3 = Mesh::load_obj_from_memory(include_bytes!("../assets/meshes/box.obj")).unwrap();

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
            Transform::new(
                Translation::new(0.0, -1.2, 0.0),
                Rotation::default(),
                Scaling::new(100.0, 0.1, 100.0),
            ),
            material.clone(),
            mesh_3.clone(),
        ));

    app.borrow()
        .get_world()
        .borrow()
        .get_camera()
        .borrow_mut()
        .get_transform_mut()
        .set_translation(Translation::new(1.5, 0.0, 6.0));

    let point_light = PointLight::new();
    point_light.borrow_mut().color = Color::from_rgb(255, 255, 255);
    point_light.borrow_mut().intensity = 4.0;
    point_light.borrow_mut().range = 20.0;
    point_light
        .borrow_mut()
        .transform
        .set_translation(Translation::new(5.0, 6.0, 3.0));
    app.borrow()
        .get_world()
        .borrow_mut()
        .add_light(point_light.clone());

    app.borrow_mut().init_camera_tickable();

    app.borrow_mut().run();
}
