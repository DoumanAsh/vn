use amethyst::prelude::{Builder, World};
use amethyst::renderer::SpriteSheetHandle;

use game::graphics::{Sprite, TextureLoader};

#[derive(Clone)]
pub struct Sprites {
    pub kaoru: [SpriteSheetHandle; 1],
}

impl Sprites {
    fn new(world: &mut World) -> Self {
        Self {
            kaoru: [
                Sprite::Path("assets/sprites/BloodyChronicles/Kaoru1.png").load(world)
            ],
        }
    }

    pub fn fetch(world: &mut World) -> Self {
        if !world.res.has_value::<Self>() {
            let this = Self::new(world);
            world.add_resource(this);
        }

        world.read_resource::<Self>().clone()
    }

    pub fn demo(&self, world: &mut World) {
        let (width, height) = {
            let dim = world.read_resource::<amethyst::renderer::ScreenDimensions>();
            (dim.width(), dim.height())
        };

        let mut transform = amethyst::core::Transform::default();
        transform.set_xyz(width * 0.5, height / 2.0, 0.0);

        let renderer = amethyst::renderer::SpriteRender {
            sprite_sheet: self.kaoru[0].clone(),
            sprite_number: 0,
        };

        world.create_entity()
             .with(renderer)
             .with(transform)
             .build();
    }
}
