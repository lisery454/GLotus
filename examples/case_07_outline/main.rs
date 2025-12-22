use std::{collections::HashMap, error::Error};

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new_with_config(AppConfig {
        anti_pixel_msaa: AntiPixel::MSAA16,
        pipeline_builder: Box::new(|| {
            let mut pipeline = Pipeline::new();
            pipeline.insert(Pass::new(
                "main".to_string(),
                RenderState::new(
                    DepthMode::new(true, true, DepthFunc::Less),
                    StencilMode::new(
                        true,
                        StencilFunc::new(StencilFuncType::Always, 1, 0xFF),
                        StencilOp::new(
                            StencilOpType::Keep,
                            StencilOpType::Keep,
                            StencilOpType::Replace,
                        ),
                        0xFF,
                    ),
                    BlendMode::default(),
                    CullFaceMode::default(),
                    PolygonMode::default(),
                ),
            ));
            pipeline.insert(Pass::new(
                "outline".to_string(),
                RenderState::new(
                    DepthMode::new(true, false, DepthFunc::LessEqual),
                    StencilMode::new(
                        true,
                        StencilFunc::new(StencilFuncType::NotEqual, 1, 0xFF),
                        StencilOp::new(
                            StencilOpType::Keep,
                            StencilOpType::Keep,
                            StencilOpType::Keep,
                        ),
                        0x00,
                    ),
                    BlendMode::default(),
                    CullFaceMode::Front,
                    PolygonMode::default(),
                ),
            ));
            pipeline
        }),
        ..Default::default()
    });

    app.borrow().build(|context| {
        let shader_1 = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .shader_manager
            .create_from_sources(
                include_str!("./assets/shaders/vs_1.vert"),
                include_str!("./assets/shaders/fs_1.frag"),
            )?;

        let shader_2 = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .shader_manager
            .create_from_sources(
                include_str!("./assets/shaders/vs_2.vert"),
                include_str!("./assets/shaders/fs_2.frag"),
            )?;

        let material_1 = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .material_manager
            .create(shader_1)?;

        let material_2 = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .material_manager
            .create(shader_2)?;

        let mesh = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .mesh_manager
            .create_from_obj_in_bytes(include_bytes!("./assets/meshes/sphere.obj"))?;

        let mesh2 = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .mesh_manager
            .create_from_obj_in_bytes(include_bytes!("./assets/meshes/sphere_no_smooth.obj"))?;

        let mesh3 = context
            .borrow()
            .asset_manager
            .borrow_mut()
            .mesh_manager
            .create_from_obj_in_bytes(include_bytes!("./assets/meshes/box.obj"))?;

        let context_borrow = context.borrow();
        let mut world = context_borrow.world.borrow_mut();

        let entity = world.spawn_entity();
        world.get_manager_mut::<RenderableComponent>().add(
            entity,
            RenderableComponent::new(
                HashMap::from([
                    ("main".to_string(), material_1),
                    ("outline".to_string(), material_2),
                ]),
                mesh,
            ),
        );
        world.get_manager_mut::<TransformComponent>().add(
            entity,
            TransformComponent::new(Transform::from_position(0.0, 0.0, 0.0)),
        );

        let entity = world.spawn_entity();
        world.get_manager_mut::<RenderableComponent>().add(
            entity,
            RenderableComponent::new(
                HashMap::from([
                    ("main".to_string(), material_1),
                    ("outline".to_string(), material_2),
                ]),
                mesh2,
            ),
        );
        world.get_manager_mut::<TransformComponent>().add(
            entity,
            TransformComponent::new(Transform::from_position(3.0, 0.0, 0.0)),
        );

        let entity = world.spawn_entity();
        world.get_manager_mut::<RenderableComponent>().add(
            entity,
            RenderableComponent::new(
                HashMap::from([
                    ("main".to_string(), material_1),
                    ("outline".to_string(), material_2),
                ]),
                mesh3,
            ),
        );
        world.get_manager_mut::<TransformComponent>().add(
            entity,
            TransformComponent::new(Transform::from_position(1.5, 0.0, 3.0)),
        );

        let camera_entity = world.spawn_entity();
        world
            .get_manager_mut::<CameraComponent>()
            .add(camera_entity, CameraComponent::new(true));
        world.get_manager_mut::<TransformComponent>().add(
            camera_entity,
            TransformComponent::new(Transform::from_position(1.5, 0.0, 6.0)),
        );

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
