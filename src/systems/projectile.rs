use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Entities, Join, Read, System, SystemData, WriteStorage};
use crate::entities::Projectile;

#[derive(SystemDesc)]
pub struct ProjectileSystem;

impl<'s> System<'s> for ProjectileSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Projectile>,
        Read<'s, Time>
    );

    fn run(&mut self, (entities, mut projectiles, time): Self::SystemData) {
        let elapsed_time = time.delta_seconds();
        for (entity, projectile) in (&*entities, &mut projectiles).join() {
            if projectile.ttl > 0.0 {
                projectile.ttl -= elapsed_time;
            }

            if projectile.ttl <= 0.0 {
                entities.delete(entity);
            }
        }
    }
}