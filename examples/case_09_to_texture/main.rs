use std::{error::Error, vec};

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new_with_config(AppConfig {
        anti_pixel_msaa: AntiPixel::MSAA16,
        ..Default::default()
    });

    app.borrow().build(|context| {
        let tex_size = 1024;
        let shader_1 = context.borrow().create_shader_from_sources(
            include_str!("./assets/shaders/vs_1.vert"),
            include_str!("./assets/shaders/fs_1.frag"),
        )?;
        let material_1 = context.borrow().create_material(shader_1)?;
        let shader_2 = context.borrow().create_shader_from_sources(
            include_str!("./assets/shaders/vs_2.vert"),
            include_str!("./assets/shaders/fs_2.frag"),
        )?;

        let topview_texture = context.borrow().create_empty_texture(
            tex_size,
            tex_size,
            WrappingMode::Repeat,
            WrappingMode::Repeat,
            FilteringMode::Nearest,
            FilteringMode::Nearest,
        )?;
        let frontview_texture = context.borrow().create_empty_texture(
            tex_size,
            tex_size,
            WrappingMode::Repeat,
            WrappingMode::Repeat,
            FilteringMode::Nearest,
            FilteringMode::Nearest,
        )?;
        let sideview_texture = context.borrow().create_empty_texture(
            tex_size,
            tex_size,
            WrappingMode::Repeat,
            WrappingMode::Repeat,
            FilteringMode::Nearest,
            FilteringMode::Nearest,
        )?;

        let topview_material = context
            .borrow()
            .get_material_builder(shader_2)?
            .with("texture1", UniformValue::Texture(0, topview_texture))
            .build();
        let frontview_material = context
            .borrow()
            .get_material_builder(shader_2)?
            .with("texture1", UniformValue::Texture(0, frontview_texture))
            .build();
        let sideview_material = context
            .borrow()
            .get_material_builder(shader_2)?
            .with("texture1", UniformValue::Texture(0, sideview_texture))
            .build();

        let topview_framebuffer =
            context
                .borrow()
                .create_framebuffer(tex_size, tex_size, topview_texture)?;
        let frontview_framebuffer =
            context
                .borrow()
                .create_framebuffer(tex_size, tex_size, frontview_texture)?;
        let sideview_framebuffer =
            context
                .borrow()
                .create_framebuffer(tex_size, tex_size, sideview_texture)?;

        let tree_mesh = context
            .borrow()
            .create_mesh_from_obj_in_bytes(include_bytes!(
                "./assets/meshes/Lowpoly_tree_sample.obj"
            ))?;

        let plane_mesh = context.borrow().create_mesh_from_position_normal_texcoord(
            &vec![0, 1, 3, 1, 2, 3],
            &vec![
                1.0, 1.0, 0.0, // 0
                1.0, -1.0, 0.0, // 1
                -1.0, -1.0, 0.0, // 2
                -1.0, 1.0, 0.0, // 3
            ],
            &vec![0.0, 0.0, 1.0].repeat(4),
            &vec![1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
        )?;

        // tree
        context.borrow().spawn_entity_with((
            RenderableComponent::new(tree_mesh)
                .with_material(DefaultPipeline::main_pass(), material_1),
            TransformComponent::new(Transform::new(
                Translation::new(0.0, -1.2, 0.0),
                Rotation::new(0.0, 0.0, 0.0),
                Scaling::new(0.1, 0.1, 0.1),
            )),
        ));

        // topview
        context.borrow().spawn_entity_with((
            RenderableComponent::new(plane_mesh)
                .with_material(DefaultPipeline::main_pass(), topview_material),
            TransformComponent::new(Transform::from_position(-3.0, -3.0, 0.0)),
        ));

        // frontview
        context.borrow().spawn_entity_with((
            RenderableComponent::new(plane_mesh)
                .with_material(DefaultPipeline::main_pass(), frontview_material),
            TransformComponent::new(Transform::from_position(0.0, -3.0, 0.0)),
        ));

        // sideview
        context.borrow().spawn_entity_with((
            RenderableComponent::new(plane_mesh)
                .with_material(DefaultPipeline::main_pass(), sideview_material),
            TransformComponent::new(Transform::from_position(3.0, -3.0, 0.0)),
        ));

        // main camera
        context.borrow().spawn_entity_with((
            TransformComponent::new(Transform::from_position(0.0, -1.5, 10.0)),
            CameraComponent::new(true),
        ));

        // topview camera
        context.borrow().spawn_entity_with((
            TransformComponent::new(Transform::new(
                Translation::new(0.0, 3.0, 0.0),
                Rotation::new(-90.0, 0.0, 0.0),
                Scaling::default(),
            )),
            CameraComponent::new(false)
                .with_target_framebuffer(topview_framebuffer)
                .with_projection_type(ProjectionType::Orthographic)
                .with_aspect_ratio(1.0)
                .with_fov(3.0)
                .with_order(-1),
        ));

        // frontview camera
        context.borrow().spawn_entity_with((
            TransformComponent::new(Transform::new(
                Translation::new(0.0, 0.0, 3.0),
                Rotation::new(0.0, 0.0, 0.0),
                Scaling::default(),
            )),
            CameraComponent::new(false)
                .with_target_framebuffer(frontview_framebuffer)
                .with_projection_type(ProjectionType::Orthographic)
                .with_aspect_ratio(1.0)
                .with_fov(3.0)
                .with_order(-1),
        ));

        // sideview camera
        context.borrow().spawn_entity_with((
            TransformComponent::new(Transform::new(
                Translation::new(3.0, 0.0, 0.0),
                Rotation::new(0.0, 90.0, 0.0),
                Scaling::default(),
            )),
            CameraComponent::new(false)
                .with_target_framebuffer(sideview_framebuffer)
                .with_projection_type(ProjectionType::Orthographic)
                .with_aspect_ratio(1.0)
                .with_fov(3.0)
                .with_order(-1),
        ));

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
