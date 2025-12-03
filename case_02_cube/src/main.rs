use glotus::{
    Entity, FilteringMode, Material, Mesh, Position, Shader, Texture2D, Transform, UniformValue,
    Vertex, WrappingMode,
};

fn main() {
    let app = glotus::App::new();
    app.borrow_mut().init_window(1400, 960);

    let shader = Shader::from_files(
        concat!(env!("CARGO_PKG_NAME"), "/assets/shaders/vs.vert"),
        concat!(env!("CARGO_PKG_NAME"), "/assets/shaders/fs.frag"),
    )
    .unwrap();

    let texture = Texture2D::from_file(
        concat!(env!("CARGO_PKG_NAME"), "/assets/textures/brick.png"),
        WrappingMode::Repeat,
        WrappingMode::Repeat,
        FilteringMode::LinearMipmapLinear,
        FilteringMode::Linear,
    )
    .unwrap();

    let material = Material::new(shader.clone());

    material
        .borrow_mut()
        .insert_uniform("texture1", UniformValue::Texture(0));
    material.borrow_mut().insert_textures(0u32, texture.clone());

    let mesh = Mesh::new(
        vec![
            // back
            Vertex::from_position_and_tex_coords(-0.5, -0.5, -0.5, 0.0, 0.0),
            Vertex::from_position_and_tex_coords(0.5, -0.5, -0.5, 1.0, 0.0),
            Vertex::from_position_and_tex_coords(0.5, 0.5, -0.5, 1.0, 1.0),
            Vertex::from_position_and_tex_coords(0.5, 0.5, -0.5, 1.0, 1.0),
            Vertex::from_position_and_tex_coords(-0.5, 0.5, -0.5, 0.0, 1.0),
            Vertex::from_position_and_tex_coords(-0.5, -0.5, -0.5, 0.0, 0.0),
            // front
            Vertex::from_position_and_tex_coords(-0.5, -0.5, 0.5, 0.0, 0.0),
            Vertex::from_position_and_tex_coords(0.5, -0.5, 0.5, 1.0, 0.0),
            Vertex::from_position_and_tex_coords(0.5, 0.5, 0.5, 1.0, 1.0),
            Vertex::from_position_and_tex_coords(0.5, 0.5, 0.5, 1.0, 1.0),
            Vertex::from_position_and_tex_coords(-0.5, 0.5, 0.5, 0.0, 1.0),
            Vertex::from_position_and_tex_coords(-0.5, -0.5, 0.5, 0.0, 0.0),
            // left
            Vertex::from_position_and_tex_coords(-0.5, 0.5, 0.5, 1.0, 0.0),
            Vertex::from_position_and_tex_coords(-0.5, 0.5, -0.5, 1.0, 1.0),
            Vertex::from_position_and_tex_coords(-0.5, -0.5, -0.5, 0.0, 1.0),
            Vertex::from_position_and_tex_coords(-0.5, -0.5, -0.5, 0.0, 1.0),
            Vertex::from_position_and_tex_coords(-0.5, -0.5, 0.5, 0.0, 0.0),
            Vertex::from_position_and_tex_coords(-0.5, 0.5, 0.5, 1.0, 0.0),
            // right
            Vertex::from_position_and_tex_coords(0.5, 0.5, 0.5, 1.0, 0.0),
            Vertex::from_position_and_tex_coords(0.5, 0.5, -0.5, 1.0, 1.0),
            Vertex::from_position_and_tex_coords(0.5, -0.5, -0.5, 0.0, 1.0),
            Vertex::from_position_and_tex_coords(0.5, -0.5, -0.5, 0.0, 1.0),
            Vertex::from_position_and_tex_coords(0.5, -0.5, 0.5, 0.0, 0.0),
            Vertex::from_position_and_tex_coords(0.5, 0.5, 0.5, 1.0, 0.0),
            // down
            Vertex::from_position_and_tex_coords(-0.5, -0.5, -0.5, 0.0, 1.0),
            Vertex::from_position_and_tex_coords(0.5, -0.5, -0.5, 1.0, 1.0),
            Vertex::from_position_and_tex_coords(0.5, -0.5, 0.5, 1.0, 0.0),
            Vertex::from_position_and_tex_coords(0.5, -0.5, 0.5, 1.0, 0.0),
            Vertex::from_position_and_tex_coords(-0.5, -0.5, 0.5, 0.0, 0.0),
            Vertex::from_position_and_tex_coords(-0.5, -0.5, -0.5, 0.0, 1.0),
            // up
            Vertex::from_position_and_tex_coords(-0.5, 0.5, -0.5, 0.0, 1.0),
            Vertex::from_position_and_tex_coords(0.5, 0.5, -0.5, 1.0, 1.0),
            Vertex::from_position_and_tex_coords(0.5, 0.5, 0.5, 1.0, 0.0),
            Vertex::from_position_and_tex_coords(0.5, 0.5, 0.5, 1.0, 0.0),
            Vertex::from_position_and_tex_coords(-0.5, 0.5, 0.5, 0.0, 0.0),
            Vertex::from_position_and_tex_coords(-0.5, 0.5, -0.5, 0.0, 1.0),
        ],
        vec![],
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
        .set_position(Position::new(0.0, 0.0, 10.0));

    app.borrow_mut().run();
}
