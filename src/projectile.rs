use bevy::prelude::*;

pub struct Projectile {
    pub damage: i32,
    pub ttl: f32,
}

impl Projectile {
    pub fn arrow() -> Projectile {
        Projectile {
            damage: 12,
            ttl: 2.,
        }
    }
}

pub fn expire_projectiles(
    mut commands: Commands,
    entity: Entity,
    _projectile: &Projectile,
    transform: &Transform,
    timer: &Timer,
) {
    if timer.finished {
        // println!("Projectile has expired at position: {:?}", transform);
        commands.despawn(entity);
    }
    // println!("Projectile position: {:?}", transform);
}
