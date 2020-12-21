use crate::data;
use bevy::prelude::*;

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub enum ProjectileType {
    Arrow,
}

#[derive(Clone)]
pub struct Projectile {
    pub damage: i32,
    pub ttl: f32,
    pub piercing: bool,
    pub speed: f32,
}

impl Projectile {
    pub fn new(damage: i32, ttl: f32, piercing: bool, speed: f32) -> Self {
        Projectile {
            damage,
            ttl,
            piercing,
            speed,
        }
    }

    pub fn arrow() -> Projectile {
        data::get_projectile_info(ProjectileType::Arrow)
    }
}

pub fn expire_projectiles(
    commands: &mut Commands,
    projectiles: Query<(&Entity, &Projectile, &Transform, &Timer)>,
) {
    for (entity, _projectile, _transform, timer) in projectiles.iter() {
        if timer.just_finished() {
            // println!("Projectile has expired at position: {:?}", transform);
            commands.despawn(*entity);
        }
        // println!("Projectile position: {:?}", transform);
    }
}
