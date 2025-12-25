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
            WrappingMode::Repeat,
            WrappingMode::Repeat,
            FilteringMode::LinearMipmapLinear,
            FilteringMode::Linear,
        )?;

        let texture_specular = context.borrow().create_texture_from_byte(
            include_bytes!("./assets/textures/texture_specular.png"),
            WrappingMode::Repeat,
            WrappingMode::Repeat,
            FilteringMode::LinearMipmapLinear,
            FilteringMode::Linear,
        )?;

        let material = context.borrow().create_material(shader)?;

        context.borrow().insert_uniform_to_material(
            material,
            "material.diffuse_texture",
            UniformValue::Texture(0, texture_diffuse),
        );
        context.borrow().insert_uniform_to_material(
            material,
            "material.specular_texture",
            UniformValue::Texture(1, texture_specular),
        );
        context.borrow().insert_uniform_to_material(
            material,
            "material.ambient_factor",
            UniformValue::Vector3([0.2, 0.2, 0.2]),
        );
        context.borrow().insert_uniform_to_material(
            material,
            "material.diffuse_factor",
            UniformValue::Vector3([1.0, 1.0, 1.0]),
        );
        context.borrow().insert_uniform_to_material(
            material,
            "material.specular_factor",
            UniformValue::Vector3([0.8, 0.8, 0.8]),
        );
        context.borrow().insert_uniform_to_material(
            material,
            "material.specular_shininess",
            UniformValue::Float(256.0),
        );

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

                    let entity = context.borrow().spawn_entity();
                    context.borrow().add_component(
                        entity,
                        RenderableComponent::new(mesh)
                            .with_material(DefaultPipeline::main_pass(), material),
                    );
                    context.borrow().add_component(
                        entity,
                        TransformComponent::new(Transform::from_position(
                            3.0 * (i as f32),
                            3.0 * (j as f32),
                            3.0 * (k as f32),
                        )),
                    );
                }
            }
        }

        let camera_entity = context.borrow().spawn_entity();
        context
            .borrow()
            .add_component(camera_entity, CameraComponent::new(true));
        context.borrow().add_component(
            camera_entity,
            TransformComponent::new(Transform::from_position(0.0, 1.0, 4.0)),
        );

        // 灯光
        let point_light_entity = context.borrow().spawn_entity();
        let mut point_light = PointLight::new();
        point_light.color = Color::from_rgb(0, 255, 0);
        point_light.intensity = 3.0;
        point_light.range = 10.0;
        context
            .borrow()
            .add_component(point_light_entity, LightComponent::new(point_light));
        context.borrow().add_component(
            point_light_entity,
            TransformComponent::new(Transform::from_position(0.0, 0.0, 0.0)),
        );

        let directional_light_entity = context.borrow().spawn_entity();
        let mut directional_light = DirectionalLight::new();
        directional_light.color = Color::from_rgb(255, 0, 0);
        context.borrow().add_component(
            directional_light_entity,
            LightComponent::new(directional_light),
        );
        let transform = Transform::new(
            Translation::default(),
            Rotation::new(0.0, 180.0, 0.0),
            Scaling::default(),
        );
        context
            .borrow()
            .add_component(directional_light_entity, TransformComponent::new(transform));

        let spot_light_entity = context.borrow().spawn_entity();
        let mut spot_light = SpotLight::new();
        spot_light.color = Color::from_rgb(0, 0, 255);
        context
            .borrow()
            .add_component(spot_light_entity, LightComponent::new(spot_light));
        let transform = Transform::new(
            Translation::new(0.0, 0.0, 8.0),
            Rotation::default(),
            Scaling::default(),
        );
        context
            .borrow()
            .add_component(spot_light_entity, TransformComponent::new(transform));

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
