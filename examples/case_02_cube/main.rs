use std::error::Error;

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new();

    app.borrow().build(|context| {
        let shader = context.borrow().create_shader_from_sources(
            include_str!("./assets/shaders/vs.vert"),
            include_str!("./assets/shaders/fs.frag"),
        )?;

        let texture = context.borrow().create_texture_2d_from_bytes(
            include_bytes!("./assets/textures/brick.png"),
            TextureConfig::new()
                .with_wrapping(WrappingMode::Repeat, WrappingMode::Repeat)
                .with_filtering(FilteringMode::LinearMipmapLinear, FilteringMode::Linear),
        )?;

        let material = context
            .borrow()
            .get_material_builder(shader)?
            .with("texture1", UniformValue::Texture2D(0, texture))
            .build();

        let mesh = context.borrow().create_mesh_from_positions_uvs(
            (0..36).collect(),
            vec![
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
            vec![
                0.0, 0.0, // 0
                1.0, 0.0, // 1
                1.0, 1.0, // 2
                1.0, 1.0, // 2
                0.0, 1.0, // 3
                0.0, 0.0, // 0
            ]
            .repeat(6),
        )?;

        // 渲染物体
        context.borrow().spawn_entity_with((
            Renderable::new(mesh).with_material(DefaultPipeline::main_pass(), material),
            Transform::default(),
        ));

        // 相机
        context
            .borrow()
            .spawn_entity_with((Camera::new(true), Transform::from_position(0.0, 0.0, 10.0)));

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
