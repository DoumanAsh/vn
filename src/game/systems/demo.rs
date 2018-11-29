use amethyst::core::Transform;
use amethyst::renderer::{SpriteRender, Flipped};
use amethyst::core::timing::Time;
use amethyst::ecs::{Entities, Join, System, WriteStorage, Read, ReadStorage};

const PERIOD: f32 = 2.5; //seconds

use crate::utils::ResultExt;
use crate::game::components::camera;

#[derive(Default)]
pub struct Demo {
    pub timer: f32,
}

pub const NAME: &'static str = "Demo-System";

impl<'s> System<'s> for Demo {
    type SystemData = (Entities<'s>, WriteStorage<'s, Transform>, ReadStorage<'s, SpriteRender>, WriteStorage<'s, Flipped>, Read<'s, Time>);

    fn run(&mut self, (entities, mut transforms, sprites, mut flips, time): Self::SystemData) {
        self.timer += time.delta_seconds();

        if self.timer < PERIOD {
            return;
        }

        self.timer = 0.0;

        for (entity, transform, sprite) in (&*entities, &mut transforms, &sprites).join() {
            if flips.contains(entity) {
                flips.remove(entity);
                transform.move_right(camera::WIDTH * 0.5);
            } else {
                flips.insert(entity, Flipped::Horizontal).unreach_err();
                transform.move_left(camera::WIDTH * 0.5);
            }
        }
    }
}
