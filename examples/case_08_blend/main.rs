use std::error::Error;

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new_with_config(AppConfig {
        anti_pixel: AntiPixel::MSAA16,
        ..Default::default()
    });

    app.borrow().build(|context| {
        let transparent_texture_shader = context.borrow().with_sdr_mgr(|m| {
            m.create(ShaderConfig::new_vert_frag(
                ShaderInput::Source(include_str!("./assets/shaders/grass_vs.vert").to_string()),
                ShaderInput::Source(include_str!("./assets/shaders/grass_fs.frag").to_string()),
            ))
        })?;

        let simple_solid_shader = context.borrow().with_sdr_mgr(|m| {
            m.create(ShaderConfig::new_vert_frag(
                ShaderInput::Source(include_str!("./assets/shaders/simple_vs.vert").to_string()),
                ShaderInput::Source(include_str!("./assets/shaders/simple_fs.frag").to_string()),
            ))
        })?;

        let grass_texture = context.borrow().with_tex_mgr(|m| {
            m.create_from_bytes(
                include_bytes!("./assets/textures/grass.png"),
                TextureConfig::common(
                    WrappingMode::ClampToEdge,
                    WrappingMode::ClampToEdge,
                    FilteringMode::LinearMipmapLinear,
                    FilteringMode::Linear,
                    FormatType::SRGBA,
                ),
            )
        })?;

        let window_texture = context.borrow().with_tex_mgr(|m| {
            m.create_from_bytes(
                include_bytes!("./assets/textures/window.png"),
                TextureConfig::common(
                    WrappingMode::ClampToEdge,
                    WrappingMode::ClampToEdge,
                    FilteringMode::LinearMipmapLinear,
                    FilteringMode::Linear,
                    FormatType::SRGBA,
                ),
            )
        })?;

        let transparent_grass_material = context.borrow().with_mat_mgr(|m| {
            m.get_builder(transparent_texture_shader)?
                .with("texture1", UniformValue::Texture(0, grass_texture))
                .build()
        })?;

        let transparent_window_material = context.borrow().with_mat_mgr(|m| {
            m.get_builder(transparent_texture_shader)?
                .with("texture1", UniformValue::Texture(0, window_texture))
                .build()
        })?;

        let solid_material = context
            .borrow()
            .with_mat_mgr(|m| m.create(simple_solid_shader))?;

        let plane_mesh = context.borrow().with_msh_mgr(|m| {
            m.create_from_positions_uvs(
                vec![0, 1, 3, 1, 2, 3],
                vec![
                    1.0, 1.0, -5.0, // 0
                    1.0, -1.0, -5.0, // 1
                    -1.0, -1.0, -5.0, // 2
                    -1.0, 1.0, -5.0, // 3
                ],
                vec![1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
            )
        })?;

        let box_mesh = context
            .borrow()
            .with_msh_mgr(|m| m.create_from_obj_bytes(include_bytes!("./assets/meshes/box.obj")))?;

        context.borrow().with_world(|w| {
            w.spawn_entity_with((Camera::new(true), Transform::from_position(0.0, 0.0, 6.0)));

            w.spawn_entity_with((
                Renderable::new(plane_mesh).with_material(
                    DefaultPipeline::transparent_pass(),
                    transparent_grass_material,
                ),
                Transform::from_position(0.0, 0.0, 0.0),
            ));

            w.spawn_entity_with((
                Renderable::new(plane_mesh).with_material(
                    DefaultPipeline::transparent_pass(),
                    transparent_window_material,
                ),
                Transform::from_position(1.5, 0.0, 3.0),
            ));

            w.spawn_entity_with((
                Renderable::new(box_mesh)
                    .with_material(DefaultPipeline::main_pass(), solid_material),
                Transform::new(
                    Translation::new(0.0, -1.2, 0.0),
                    Default::default(),
                    Scaling::new(100.0, 0.1, 100.0),
                ),
            ));

            w.spawn_entity_with((
                Renderable::new(plane_mesh).with_material(
                    DefaultPipeline::transparent_pass(),
                    transparent_window_material,
                ),
                Transform::from_position(0.4, -1.0, -2.0),
            ));

            w.spawn_entity_with((
                Renderable::new(plane_mesh).with_material(
                    DefaultPipeline::transparent_pass(),
                    transparent_window_material,
                ),
                Transform::from_position(1.9, 1.0, 1.0),
            ));
        });
        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
