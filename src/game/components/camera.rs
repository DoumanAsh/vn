use amethyst::prelude::{Builder, World};

pub const WIDTH: f32 = 5000.0;
pub const HEIGHT: f32 = 3000.0;

pub struct Camera {
    inner: amethyst::ecs::Entity,
}

impl Camera {
    pub fn new(world: &mut World) -> Self {
        //TODO: think about camera sizes
        let camera = amethyst::renderer::Projection::orthographic(0.0, WIDTH, 0.0, HEIGHT);
        let camera = amethyst::renderer::Camera::from(camera);
        let mut transform = amethyst::core::Transform::default();
        transform.set_z(100.0);

        let inner = world.create_entity()
                         .with(camera)
                         .with(transform)
                         .build();


        Self {
            inner
        }
    }

    pub fn destroy(self, world: &mut World) {
        let _ = world.delete_entity(self.inner);
    }
}
