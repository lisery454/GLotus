use std::{cell::RefCell, error::Error, rc::Rc};

use glotus::*;
use rand::Rng;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new_with_config(AppConfig {
        anti_pixel: AntiPixel::MSAA4,
        instancing: true,
        bg_color: Color::BLACK,
        ..Default::default()
    });

    app.borrow().build(|context| {
        let shader = context.borrow().with_sdr_mgr(|m| {
            m.create(ShaderConfig::new_vert_frag(
                ShaderInput::Source(include_str!("./assets/shaders/vs.vert").to_string()),
                ShaderInput::Source(include_str!("./assets/shaders/fs.frag").to_string()),
            ))
        })?;

        let texture = context.borrow().with_tex_mgr(|m| {
            m.create_from_bytes(
                include_bytes!("./assets/textures/rock.png"),
                TextureConfig::new()
                    .with_wrapping(WrappingMode::Repeat, WrappingMode::Repeat)
                    .with_filtering(FilteringMode::LinearMipmapLinear, FilteringMode::Linear),
            )
        })?;

        let material = context.borrow().with_mat_mgr(|m| {
            m.get_builder(shader)?
                .with("texture_diffuse1", UniformValue::Texture(0, texture))
                .build()
        })?;

        let mesh = context.borrow().with_msh_mgr(|m| {
            m.create_from_obj_bytes(include_bytes!("./assets/meshes/rock.obj"))
        })?;

        context.borrow().with_world(|w| {
            let mut rng = rand::rng();
            for i in 0..10000 {
                let s = rng.random_range(0.1..0.4);
                let r1 = rng.random_range(0.0..360.0);
                let r2 = rng.random_range(0.0..360.0);
                let r3 = rng.random_range(0.0..360.0);
                w.spawn_entity_with((
                    Renderable::new(mesh).with_material(DefaultPipeline::main_pass(), material),
                    Transform::new(
                        Default::default(),
                        Rotation::new(r1, r2, r3),
                        Scaling::new(s, s, s),
                    ),
                    Scriptable::new().with(RockMove::new(i)),
                ));
            }

            w.spawn_entity_with((
                Transform::new(
                    Translation::new(0.0, 80.0, 0.0),
                    Rotation::new(-90.0, 0.0, 0.0),
                    Scaling::default(),
                ),
                Camera::new(true).with_far_plane(200.0),
            ));
        });

        Ok(())
    })?;

    app.borrow_mut().run();

    Ok(())
}

struct RockMove {
    id: f32,
    total_time: f32,
}

impl RockMove {
    pub fn new(id: u32) -> Self {
        Self {
            id: id as f32,
            total_time: 0.0,
        }
    }
}

impl IBehavior for RockMove {
    fn on_fixed_update(&mut self, entity: EntityHandle, context: Rc<RefCell<AppContext>>, dt: f32) {
        let ctx = context.borrow();
        let world = ctx.world.borrow();
        let mut transform_mgr = world.get_manager_mut::<Transform>();
        self.total_time += dt;

        let dot = self.id * 12.9898;
        let fract = (dot.sin() * 43758.5453).fract().abs();
        let d = (self.id) * 3.7 % 20.0 + fract.sin() * 2.3 + 13.0;
        let a = self.total_time * 0.04 * fract.cos() + self.id * 0.37;
        let x = a.cos() * d;
        let z = a.sin() * d;

        // 修改变换位置
        if let Some(transform) = transform_mgr.get_mut(entity) {
            transform.get_translation_mut().set_x(x);
            transform.get_translation_mut().set_y(0.0);
            transform.get_translation_mut().set_z(z);
        }
    }
}
