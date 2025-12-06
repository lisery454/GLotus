use std::{cell::RefCell, rc::Rc};

use glotus::{
    AppConfig, Color, Entity, FilteringMode, ITickable, Material, Mesh, PointLight, Position,
    Shader, Texture2D, Transform, UniformValue, Vertex, WrappingMode,
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
        vec![],
    );

    let entity = Entity::new(
        Transform::from_position(0.0, 0.0, 0.0),
        material.clone(),
        mesh.clone(),
    );

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
    point_light.borrow_mut().color = Color::from_rgb(0, 255, 0);
    point_light.borrow_mut().intensity = 1.0;
    point_light.borrow_mut().range = 10.0;
    point_light
        .borrow_mut()
        .transform
        .set_position(Position::new(5.0, 0.0, 0.0));
    app.borrow()
        .get_world()
        .borrow_mut()
        .add_light(point_light.clone());
    let light_tickable = LightTickable::new(point_light.clone());
    app.borrow_mut()
        .get_ticker()
        .borrow_mut()
        .add_tickable(light_tickable);

    app.borrow_mut().run();
}

struct LightTickable {
    light: Rc<RefCell<PointLight>>,
    hue: f32,
    total_time: f32,
}

impl LightTickable {
    pub fn new(light: Rc<RefCell<PointLight>>) -> Box<Self> {
        Box::new(Self {
            light,
            hue: 0.0,
            total_time: 0.0,
        })
    }
}

impl ITickable for LightTickable {
    fn tick(
        &mut self,
        delta_time: f32,
        _input_state: Rc<RefCell<glotus::input::input_state::InputState>>,
    ) {
        self.total_time += delta_time;
        self.hue = (self.hue + delta_time * 0.1) % 1.0; // 0.2 = 速度
        let color = Color::from_hsv(self.hue, 1.0, 1.0);
        self.light.borrow_mut().color = color;

        let x = self.total_time.cos() * 5.0;
        let y = self.total_time.sin() * 5.0 * (self.total_time * 0.5).sin();
        let z = self.total_time.sin() * 5.0 * (self.total_time * 0.5).cos();

        let transform = &mut self.light.borrow_mut().transform;

        transform.get_position_mut().set_x(x);
        transform.get_position_mut().set_y(y);
        transform.get_position_mut().set_z(z);
    }
}
