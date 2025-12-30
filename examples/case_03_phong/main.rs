use std::error::Error;

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

        let texture_diffuse = context.borrow().create_texture_from_byte(
            include_bytes!("./assets/textures/texture_diffuse.png"),
            TextureConfig::new()
                .with_wrapping(WrappingMode::Repeat, WrappingMode::Repeat)
                .with_filtering(FilteringMode::LinearMipmapLinear, FilteringMode::Linear),
        )?;

        let texture_specular = context.borrow().create_texture_from_byte(
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

        let mesh = context.borrow().create_mesh_from_position_normal_texcoord(
            &(0..36).collect(),
            &vec![
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
            &[
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
            &vec![
                0.0, 0.0, // 0
                1.0, 0.0, // 1
                1.0, 1.0, // 2
                1.0, 1.0, // 2
                0.0, 1.0, // 3
                0.0, 0.0, // 0
            ]
            .repeat(6),
        )?;

        for i in -1..2 {
            for j in -1..2 {
                for k in -1..2 {
                    if i == 0 && j == 0 && k == 0 {
                        continue;
                    }

                    context.borrow().spawn_entity_with((
                        Renderable::new(mesh)
                            .with_material(DefaultPipeline::main_pass(), material),
                        Transform::from_position(
                            3.0 * (i as f32),
                            3.0 * (j as f32),
                            3.0 * (k as f32),
                        ),
                    ));
                }
            }
        }

        context
            .borrow()
            .spawn_entity_with((Camera::new(true), Transform::from_position(0.0, 1.0, 4.0)));

        // 灯光
        context.borrow().spawn_entity_with((
            Transform::from_position(0.0, 0.0, 0.0),
            Light::from(
                PointLight::new()
                    .with_color(Color::GREEN)
                    .with_intensity(3.0)
                    .with_range(10.0),
            ),
        ));

        context.borrow().spawn_entity_with((
            Light::from(DirectionalLight::new().with_color(Color::RED)),
            Transform::new(
                Translation::default(),
                Rotation::new(0.0, 180.0, 0.0),
                Scaling::default(),
            ),
        ));

        context.borrow().spawn_entity_with((
            Light::from(SpotLight::new().with_color(Color::BLUE)),
            Transform::new(
                Translation::new(0.0, 0.0, 8.0),
                Rotation::default(),
                Scaling::default(),
            ),
        ));

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
