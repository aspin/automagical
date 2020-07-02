use amethyst::core::Transform;
use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use crate::components::physics::Physics;

#[derive(SystemDesc)]
pub struct ResourcePhysicsSystem;

impl<'s> System<'s> for ResourcePhysicsSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Physics>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut transforms, physics, time): Self::SystemData) {
        for (physic, transform) in (&physics, &mut transforms).join() {
            let (x, y, _) = physic.generate_movement(time.delta_seconds());
            transform.prepend_translation_x(x);
            transform.prepend_translation_y(y);
        }
    }
}
