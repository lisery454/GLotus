use glotus::*;

fn main() {
    let app = glotus::App::new_with_config(AppConfig {
        // bg_color: [0.0, 0.0, 0.0],
        anti_pixel_msaa: AntiPixel::MSAA16,
        pipeline_builder: Box::new(|| {
            let mut pipeline = Pipeline::new();
            pipeline.insert(Pass::new(
                "main",
                RenderState {
                    depth_test: true,
                    depth_write: true,
                    stencil_test: true,
                    stencil_func: Some(StencilFunc {
                        func: StencilFuncType::Always,
                        ref_value: 1,
                        mask: 0xFF,
                    }),
                    stencil_op: Some(StencilOp {
                        sfail: StencilOpType::Keep,
                        dpfail: StencilOpType::Keep,
                        dppass: StencilOpType::Replace,
                    }),
                    stencil_write_mask: Some(0xFF),
                    ..Default::default()
                },
            ));
            pipeline.insert(Pass::new(
                "outline",
                RenderState {
                    depth_test: true,
                    depth_write: false,
                    depth_func: DepthFunc::LessEqual,
                    stencil_test: true,
                    stencil_func: Some(StencilFunc {
                        func: StencilFuncType::NotEqual,
                        ref_value: 1,
                        mask: 0xFF,
                    }),
                    stencil_op: Some(StencilOp {
                        sfail: StencilOpType::Keep,
                        dpfail: StencilOpType::Keep,
                        dppass: StencilOpType::Keep,
                    }),
                    stencil_write_mask: Some(0x00),       // 禁止写入
                    cull_face: Some(CullFaceMode::Front), // 剔除正面
                    ..Default::default()
                },
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
