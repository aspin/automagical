use amethyst::core::Transform;
use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use crate::entities::tower::Tower;

#[derive(SystemDesc)]
pub struct DefenseSystem;

impl<'s> System<'s> for DefenseSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Tower>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut transforms, mut towers, time): Self::SystemData) {
        for tower in (&mut towers).join() {
            if let Some(time_since_last_shot) = tower.time_since_last_shot {
                let updated_time = time_since_last_shot - time.delta_seconds();
                if updated_time > 0. {
                    tower.time_since_last_shot.replace(updated_time);
                } else {
                    tower.time_since_last_shot.take();
                }
            } else {
                tower.time_since_last_shot.replace(tower.fire_rate);
                // create the projectile
            }
        }
    }
}
