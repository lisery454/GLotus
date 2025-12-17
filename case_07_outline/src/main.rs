use glotus::*;

fn main() {
    let app = glotus::App::new_with_config(AppConfig {
        // bg_color: [0.0, 0.0, 0.0],
        anti_pixel_msaa: AntiPixel::MSAA16,
        pipeline_builder: Box::new(|| {
            let mut pipeline = Pipeline::new();
            pipeline.insert(Pass::new(
                "main",
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
                "outline",
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

    let shader_1 = Shader::from_sources(
        include_str!("../assets/shaders/vs_1.vert"),
        include_str!("../assets/shaders/fs_1.frag"),
    )
    .unwrap();
    let material_1 = Material::new(shader_1.clone());
    let shader_2 = Shader::from_sources(
        include_str!("../assets/shaders/vs_2.vert"),
        include_str!("../assets/shaders/fs_2.frag"),
    )
    .unwrap();
    let material_2 = Material::new(shader_2.clone());
    let material_group = MaterialGroup::new();
    material_group.borrow_mut().insert("main", material_1);
    material_group.borrow_mut().insert("outline", material_2);

    let mesh = Mesh::load_obj_from_memory(include_bytes!("../assets/meshes/sphere_no_smooth.obj"))
        .unwrap();

    app.borrow()
        .get_world()
        .borrow_mut()
        .add_entity(Entity::new(
            Transform::from_position(0.0, 0.0, 0.0),
            material_group.clone(),
            mesh.clone(),
        ));

    app.borrow()
        .get_world()
        .borrow_mut()
        .add_entity(Entity::new(
            Transform::from_position(1.5, 0.0, 0.0),
            material_group.clone(),
            mesh.clone(),
        ));

    app.borrow()
        .get_world()
        .borrow_mut()
        .add_entity(Entity::new(
            Transform::from_position(1.0, 0.0, 1.5),
            material_group.clone(),
            mesh.clone(),
        ));

    app.borrow()
        .get_world()
        .borrow()
        .get_camera()
        .borrow_mut()
        .get_transform_mut()
        .set_translation(Translation::new(0.0, 0.0, 6.0));

    app.borrow_mut().init_camera_tickable();

    app.borrow_mut().run();
}
