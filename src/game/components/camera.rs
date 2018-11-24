use amethyst::prelude::{Builder, World};

pub struct Camera {
    inner: amethyst::ecs::Entity,
}

impl Camera {
    pub fn new(world: &mut World) -> Self {
        //TODO: think about camera sizes
        let camera = amethyst::renderer::Projection::orthographic(0.0, 3000.0, 0.0, 3000.0);
        let camera = amethyst::renderer::Camera::from(camera);
        let mut transform = amethyst::core::Transform::default();
        transform.set_z(1.0);

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
