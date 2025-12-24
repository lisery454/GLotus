use std::error::Error;

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new();

    app.borrow().build(|context| {
        let shader = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .shader_manager
            .create_from_sources(
                include_str!("./assets/shaders/vs.vert"),
                include_str!("./assets/shaders/fs.frag"),
            )?;

        let texture = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .texture_manager
            .create_from_byte(
                include_bytes!("./assets/textures/brick.png"),
                WrappingMode::Repeat,
                WrappingMode::Repeat,
                FilteringMode::LinearMipmapLinear,
                FilteringMode::Linear,
            )?;

        let material = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .material_manager
            .create(shader)?;

        context
            .borrow()
            .asset_manager
            .borrow_mut()
            .material_manager
            .insert_uniform(
                material,
                "texture1",
                UniformValue::Texture(0, texture),
            );

        let mesh = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .mesh_manager
            .create_from_position_texcoord(
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

        let context_borrow = context.borrow();
        let mut world = context_borrow.world.borrow_mut();
        // 渲染物体
        let entity = world.spawn_entity();
        world.get_manager_mut::<RenderableComponent>().add(
            entity,
            RenderableComponent::new(mesh)
                .with_material(DefaultPipeline::main_pass(), material),
        );
        world
            .get_manager_mut::<TransformComponent>()
            .add(entity, TransformComponent::new(Transform::default()));
        // 相机
        let camera_entity = world.spawn_entity();
        world
            .get_manager_mut::<CameraComponent>()
            .add(camera_entity, CameraComponent::new(true));
        world.get_manager_mut::<TransformComponent>().add(
            camera_entity,
            TransformComponent::new(Transform::from_position(0.0, 0.0, 10.0)),
        );

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
