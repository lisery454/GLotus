use std::error::Error;

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = glotus::App::new_with_config(AppConfig {
        bg_color: Color::from_rgb(0, 0, 0),
        // resolution: Resolution::new(400, 300),
        ..Default::default()
    });

    app.borrow().build(|context| {
        let sky_box_texture = context.borrow().with_tex_mgr(|m| {
            m.create_cube_map_from_bytes(
                [
                    include_bytes!("./assets/textures/right.png"),
                    include_bytes!("./assets/textures/left.png"),
                    include_bytes!("./assets/textures/top.png"),
                    include_bytes!("./assets/textures/bottom.png"),
                    include_bytes!("./assets/textures/front.png"),
                    include_bytes!("./assets/textures/back.png"),
                ],
                TextureConfig::common(
                    WrappingMode::ClampToEdge,
                    WrappingMode::ClampToEdge,
                    FilteringMode::LinearMipmapLinear,
                    FilteringMode::Linear,
                    FormatType::SRGBA,
                ),
            )
        })?;

        let sky_box_shader = context.borrow().with_sdr_mgr(|m| {
            m.create(ShaderConfig::new_vert_frag(
                ShaderInput::Source(include_str!("./assets/shaders/skybox.vert").to_string()),
                ShaderInput::Source(include_str!("./assets/shaders/skybox.frag").to_string()),
            ))
        })?;

        let sky_box_material = context.borrow().with_mat_mgr(|m| {
            m.get_builder(sky_box_shader)?
                .with("skybox", UniformValue::Texture(0, sky_box_texture))
                .build()
        })?;

        let refraction_shader = context.borrow().with_sdr_mgr(|m| {
            m.create(ShaderConfig::new_vert_frag(
                ShaderInput::Source(include_str!("./assets/shaders/refraction.vert").to_string()),
                ShaderInput::Source(include_str!("./assets/shaders/refraction.frag").to_string()),
            ))
        })?;
        let refraction_material = context
            .borrow()
            .with_mat_mgr(|m| m.create(refraction_shader))?;

        let reflection_shader = context.borrow().with_sdr_mgr(|m| {
            m.create(ShaderConfig::new_vert_frag(
                ShaderInput::Source(include_str!("./assets/shaders/reflection.vert").to_string()),
                ShaderInput::Source(include_str!("./assets/shaders/reflection.frag").to_string()),
            ))
        })?;
        let reflection_material = context
            .borrow()
            .with_mat_mgr(|m| m.create(reflection_shader))?;

        let box_mesh = context
            .borrow()
            .with_msh_mgr(|m| m.create_from_obj_bytes(include_bytes!("./assets/meshes/box.obj")))?;

        let ball_mesh = context.borrow().with_msh_mgr(|m| {
            m.create_from_obj_bytes(include_bytes!("./assets/meshes/ball.obj"))
        })?;

        context.borrow().with_world(|w| {
            w.spawn_entity_with((
                Renderable::new(box_mesh)
                    .with_material(DefaultPipeline::skybox_pass(), sky_box_material),
                Transform::default(),
            ));

            w.spawn_entity_with((
                Renderable::new(ball_mesh)
                    .with_material(DefaultPipeline::main_pass(), reflection_material),
                Transform::default(),
            ));

            w.spawn_entity_with((
                Renderable::new(ball_mesh)
                    .with_material(DefaultPipeline::main_pass(), refraction_material),
                Transform::from_position(5.0, 0.0, 0.0),
            ));

            w.spawn_entity_with((Camera::new(true), Transform::from_position(0.0, 1.0, 4.0)));
        });

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
