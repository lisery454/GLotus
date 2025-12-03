use glotus::{
    entity::entity::Entity,
    material::Material,
    mesh::{Mesh, Vertex},
    shader::Shader,
    transform::Transform,
};

fn main() {
    let app = glotus::App::new();
    app.borrow_mut().init_window(1400, 960);

    let shader = Shader::from_files(
        concat!(env!("CARGO_PKG_NAME"), "/assets/shaders/vs_0.vert"),
        concat!(env!("CARGO_PKG_NAME"), "/assets/shaders/fs_0.frag"),
    )
    .unwrap();

    let material = Material::new(shader.clone());

    let mesh = Mesh::new(
        vec![
            Vertex::from_position(1.0, 1.0, -5.0),
            Vertex::from_position(1.0, -1.0, -5.0),
            Vertex::from_position(-1.0, -1.0, -5.0),
            Vertex::from_position(-1.0, 1.0, -5.0),
        ],
        vec![0, 1, 3, 1, 2, 3],
    );

    let entity = Entity::new(Transform::default(), material.clone(), mesh.clone());

    app.borrow().get_world().borrow_mut().add_entity(entity.clone());

    app.borrow_mut().run();
}
