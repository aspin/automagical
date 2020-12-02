use crate::builder::CardinalDirection;
use bevy::prelude::*;

pub const ARROW_SPEED: f32 = 700.;

pub struct Projectile {
    pub damage: i32,
    pub ttl: f32,
    pub facing: CardinalDirection,
    pub piercing: bool,
}

impl Projectile {
    pub fn arrow(facing: CardinalDirection) -> Projectile {
        Projectile {
            damage: 12,
            ttl: 2.,
            facing,
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
