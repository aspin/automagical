use bevy::prelude::*;

pub const ARROW_SPEED: f32 = 400.;
pub const ARROW_OFFSET: f32 = 3.;
pub const ARROW_SPREAD: f32 = 5.;

pub struct Projectile {
    pub damage: i32,
    pub ttl: f32,
    pub piercing: bool,
}

impl Projectile {
    pub fn arrow() -> Projectile {
        Projectile {
            damage: 12,
            ttl: 2.,
            piercing: true,
        }
    }
}

pub fn expire_projectiles(
    mut commands: Commands,
    entity: Entity,
    _projectile: &Projectile,
    _transform: &Transform,
    timer: &Timer,
) {
    if timer.finished {
        // println!("Projectile has expired at position: {:?}", transform);
        commands.despawn(entity);
    }
    // println!("Projectile position: {:?}", transform);
}
