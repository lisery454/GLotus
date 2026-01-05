use std::{cell::RefCell, error::Error, rc::Rc};

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = glotus::App::new_with_config(AppConfig {
        bg_color: Color::from_rgb(0, 0, 0),
        ..Default::default()
    });

    app.borrow().build(|context| {
        let shader = context.borrow().create_shader_from_sources(
            include_str!("./assets/shaders/vs.vert"),
            include_str!("./assets/shaders/fs.frag"),
        )?;

        let texture_diffuse = context.borrow().create_texture_2d_from_bytes(
            include_bytes!("./assets/textures/texture_diffuse.png"),
            TextureConfig::new()
                .with_wrapping(WrappingMode::Repeat, WrappingMode::Repeat)
                .with_filtering(FilteringMode::LinearMipmapLinear, FilteringMode::Linear),
        )?;

        let texture_specular = context.borrow().create_texture_2d_from_bytes(
            include_bytes!("./assets/textures/texture_specular.png"),
            TextureConfig::new()
                .with_wrapping(WrappingMode::Repeat, WrappingMode::Repeat)
                .with_filtering(FilteringMode::LinearMipmapLinear, FilteringMode::Linear),
        )?;

        let material = context
            .borrow()
            .get_material_builder(shader)?
            .with(
                "material.diffuse_texture",
                UniformValue::Texture(0, texture_diffuse),
            )
            .with(
                "material.specular_texture",
                UniformValue::Texture(1, texture_specular),
            )
            .with(
                "material.ambient_factor",
                UniformValue::Vector3([0.2, 0.2, 0.2]),
            )
            .with(
                "material.diffuse_factor",
                UniformValue::Vector3([1.0, 1.0, 1.0]),
            )
            .with(
                "material.specular_factor",
                UniformValue::Vector3([0.8, 0.8, 0.8]),
            )
            .with("material.specular_shininess", UniformValue::Float(256.0))
            .build();

        let mesh = context.borrow().create_mesh_from_positions_normals_uvs(
            (0..36).collect(),
            vec![
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
            [
                [0.0, 0.0, -1.0].repeat(6),
                [0.0, 0.0, 1.0].repeat(6),
                [-1.0, 0.0, 0.0].repeat(6),
                [1.0, 0.0, 0.0].repeat(6),
                [0.0, 1.0, 0.0].repeat(6),
                [0.0, -1.0, 0.0].repeat(6),
            ]
            .into_iter()
            .flatten()
            .collect(),
            vec![
                0.0, 0.0, // 0
                1.0, 0.0, // 1
                1.0, 1.0, // 2
                1.0, 1.0, // 2
                0.0, 1.0, // 3
                0.0, 0.0, // 0
            ]
            .repeat(6),
        )?;

        context.borrow().spawn_entity_with((
            Renderable::new(mesh).with_material(DefaultPipeline::main_pass(), material),
            Transform::default(),
        ));

        context
            .borrow()
            .spawn_entity_with((Transform::from_position(0.0, 0.0, 4.0), Camera::new(true)));

        context.borrow().spawn_entity_with((
            Transform::from_position(5.0, 0.0, 0.0),
            Light::point()
                .with_color(Color::GREEN)
                .with_intensity(1.0)
                .with_range(10.0),
            Scriptable::new().with(LightTickable::new()),
        ));

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}

struct LightTickable {
    hue: f32,
    total_time: f32,
}

impl LightTickable {
    pub fn new() -> Self {
        Self {
            hue: 0.0,
            total_time: 0.0,
        }
    }
}

impl IBehavior for LightTickable {
    fn on_fixed_update(&mut self, entity: EntityHandle, context: Rc<RefCell<AppContext>>, dt: f32) {
        let ctx = context.borrow();
        let world = ctx.world.borrow();
        let mut light_mgr = world.get_manager_mut::<Light>();
        let mut transform_mgr = world.get_manager_mut::<Transform>();

        self.total_time += dt;

        self.hue = (self.hue + dt * 0.05) % 1.0;
        let new_color = Color::from_hsv(self.hue, 1.0, 1.0);

        let x = self.total_time.cos() * 5.0;
        let y = self.total_time.sin() * 5.0 * (self.total_time * 0.5).sin();
        let z = self.total_time.sin() * 5.0 * (self.total_time * 0.5).cos();

        // 修改灯光颜色

        if let Some(light) = light_mgr.get_mut(entity) {
            light.color = new_color;
        }

        // 修改变换位置
        if let Some(transform) = transform_mgr.get_mut(entity) {
            transform.get_translation_mut().set_x(x);
            transform.get_translation_mut().set_y(y);
            transform.get_translation_mut().set_z(z);
        }
    }
}
