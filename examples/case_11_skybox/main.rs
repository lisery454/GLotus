use std::error::Error;

use glotus::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = glotus::App::new_with_config(AppConfig {
        bg_color: Color::from_rgb(0, 0, 0),
        ..Default::default()
    });

    app.borrow().build(|context| {
        let sky_box_texture = context.borrow().create_cube_map_from_bytes(
            [
                include_bytes!("./assets/textures/right.jpg"),
                include_bytes!("./assets/textures/left.jpg"),
                include_bytes!("./assets/textures/top.jpg"),
                include_bytes!("./assets/textures/bottom.jpg"),
                include_bytes!("./assets/textures/front.jpg"),
                include_bytes!("./assets/textures/back.jpg"),
            ],
            TextureConfig::new()
                .with_wrapping(WrappingMode::ClampToEdge, WrappingMode::ClampToEdge)
                .with_filtering(FilteringMode::LinearMipmapLinear, FilteringMode::Linear),
        )?;

        let sky_box_shader = context.borrow().create_shader_from_sources(
            include_str!("./assets/shaders/skybox.vert"),
            include_str!("./assets/shaders/skybox.frag"),
        )?;

        let sky_box_material = context
            .borrow()
            .get_material_builder(sky_box_shader)?
            .with("skybox", UniformValue::Texture(0, sky_box_texture))
            .build();

        let refraction_shader = context.borrow().create_shader_from_sources(
            include_str!("./assets/shaders/refraction.vert"),
            include_str!("./assets/shaders/refraction.frag"),
        )?;
        let refraction_material = context.borrow().create_material(refraction_shader)?;

        let reflection_shader = context.borrow().create_shader_from_sources(
            include_str!("./assets/shaders/reflection.vert"),
            include_str!("./assets/shaders/reflection.frag"),
        )?;
        let reflection_material = context.borrow().create_material(reflection_shader)?;

        let box_mesh = context
            .borrow()
            .create_mesh_from_obj_in_bytes(include_bytes!("./assets/meshes/box.obj"))?;

        let ball_mesh = context
            .borrow()
            .create_mesh_from_obj_in_bytes(include_bytes!("./assets/meshes/ball.obj"))?;

        context.borrow().spawn_entity_with((
            Renderable::new(box_mesh)
                .with_material(DefaultPipeline::skybox_pass(), sky_box_material),
            Transform::default(),
        ));

        context.borrow().spawn_entity_with((
            Renderable::new(ball_mesh)
                .with_material(DefaultPipeline::main_pass(), reflection_material),
            Transform::default(),
        ));

        context.borrow().spawn_entity_with((
            Renderable::new(ball_mesh)
                .with_material(DefaultPipeline::main_pass(), refraction_material),
            Transform::from_position(5.0, 0.0, 0.0),
        ));

        context
            .borrow()
            .spawn_entity_with((Camera::new(true), Transform::from_position(0.0, 1.0, 4.0)));

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}
