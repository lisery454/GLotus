use std::{error::Error, vec};

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new_with_config(AppConfig {
        anti_pixel: AntiPixel::MSAA16,
        ..Default::default()
    });

    app.borrow().build(|context| {
        let tex_size = 1024;
        let shader_1 = context.borrow().with_sdr_mgr(|m| {
            m.create(ShaderConfig::new_vert_frag(
                ShaderInput::Source(include_str!("./assets/shaders/normal.vert").to_string()),
                ShaderInput::Source(include_str!("./assets/shaders/normal.frag").to_string()),
            ))
        })?;

        let material_1 = context.borrow().with_mat_mgr(|m| m.create(shader_1))?;

        let shader_2 = context.borrow().with_sdr_mgr(|m| {
            m.create(ShaderConfig::new_vert_frag(
                ShaderInput::Source(include_str!("./assets/shaders/tex.vert").to_string()),
                ShaderInput::Source(include_str!("./assets/shaders/tex.frag").to_string()),
            ))
        })?;

        let texture_config = TextureConfig::common(
            WrappingMode::ClampToEdge,
            WrappingMode::ClampToEdge,
            FilteringMode::Nearest,
            FilteringMode::Nearest,
            FormatType::SRGBA,
        );
        let texture_resolution = Resolution::new(tex_size, tex_size);

        let topview_framebuffer = context
            .borrow()
            .with_fbr_mgr(|m| m.create(texture_resolution, texture_config))?;
        let frontview_framebuffer = context
            .borrow()
            .with_fbr_mgr(|m| m.create(texture_resolution, texture_config))?;
        let sideview_framebuffer = context
            .borrow()
            .with_fbr_mgr(|m| m.create(texture_resolution, texture_config))?;

        let topview_texture = context
            .borrow()
            .with_fbr_mgr(|m| m.get_color_texture(topview_framebuffer))?;
        let topview_material = context.borrow().with_mat_mgr(|m| {
            m.get_builder(shader_2)?
                .with("texture1", UniformValue::Texture(0, topview_texture))
                .build()
        })?;

        let frontview_texture = context
            .borrow()
            .with_fbr_mgr(|m| m.get_color_texture(frontview_framebuffer))?;
        let frontview_material = context.borrow().with_mat_mgr(|m| {
            m.get_builder(shader_2)?
                .with("texture1", UniformValue::Texture(0, frontview_texture))
                .build()
        })?;

        let sideview_texture = context
            .borrow()
            .with_fbr_mgr(|m| m.get_color_texture(sideview_framebuffer))?;
        let sideview_material = context.borrow().with_mat_mgr(|m| {
            m.get_builder(shader_2)?
                .with("texture1", UniformValue::Texture(0, sideview_texture))
                .build()
        })?;

        let tree_mesh = context.borrow().with_msh_mgr(|m| {
            m.create_from_obj_bytes(include_bytes!("./assets/meshes/Lowpoly_tree_sample.obj"))
        })?;

        let plane_mesh = context.borrow().with_msh_mgr(|m| {
            m.create_from_positions_normals_uvs(
                vec![0, 1, 3, 1, 2, 3],
                vec![
                    1.0, 1.0, 0.0, // 0
                    1.0, -1.0, 0.0, // 1
                    -1.0, -1.0, 0.0, // 2
                    -1.0, 1.0, 0.0, // 3
                ],
                vec![0.0, 0.0, 1.0].repeat(4),
                vec![1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
            )
        })?;

        context.borrow().with_world(|w| {
            // tree
            w.spawn_entity_with((
                Renderable::new(tree_mesh).with_material(DefaultPipeline::main_pass(), material_1),
                Transform::new(
                    Translation::new(0.0, -1.2, 0.0),
                    Rotation::new(0.0, 0.0, 0.0),
                    Scaling::new(0.1, 0.1, 0.1),
                ),
            ));

            // topview
            w.spawn_entity_with((
                Renderable::new(plane_mesh)
                    .with_material(DefaultPipeline::main_pass(), topview_material),
                Transform::from_position(-3.0, -3.0, 0.0),
            ));

            // frontview
            w.spawn_entity_with((
                Renderable::new(plane_mesh)
                    .with_material(DefaultPipeline::main_pass(), frontview_material),
                Transform::from_position(0.0, -3.0, 0.0),
            ));

            // sideview
            w.spawn_entity_with((
                Renderable::new(plane_mesh)
                    .with_material(DefaultPipeline::main_pass(), sideview_material),
                Transform::from_position(3.0, -3.0, 0.0),
            ));

            // main camera
            w.spawn_entity_with((Transform::from_position(0.0, -1.5, 10.0), Camera::new(true)));

            // topview camera
            w.spawn_entity_with((
                Transform::new(
                    Translation::new(0.0, 3.0, 0.0),
                    Rotation::new(-90.0, 0.0, 0.0),
                    Scaling::default(),
                ),
                Camera::new(false)
                    .with_target_framebuffer(topview_framebuffer)
                    .with_projection_type(ProjectionType::Orthographic)
                    .with_aspect_ratio(1.0)
                    .with_fov(200.0)
                    .with_order(-1),
            ));

            // frontview camera
            w.spawn_entity_with((
                Transform::new(
                    Translation::new(0.0, 0.0, 3.0),
                    Rotation::new(0.0, 0.0, 0.0),
                    Scaling::default(),
                ),
                Camera::new(false)
                    .with_target_framebuffer(frontview_framebuffer)
                    .with_projection_type(ProjectionType::Orthographic)
                    .with_aspect_ratio(1.0)
                    .with_fov(200.0)
                    .with_order(-1),
            ));

            // sideview camera
            w.spawn_entity_with((
                Transform::new(
                    Translation::new(3.0, 0.0, 0.0),
                    Rotation::new(0.0, 90.0, 0.0),
                    Scaling::default(),
                ),
                Camera::new(false)
                    .with_target_framebuffer(sideview_framebuffer)
                    .with_projection_type(ProjectionType::Orthographic)
                    .with_aspect_ratio(1.0)
                    .with_fov(200.0)
                    .with_order(-1),
            ));
        });

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
