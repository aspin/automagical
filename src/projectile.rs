use bevy::prelude::*;
use crate::data;

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub enum ProjectileType {
    Arrow
}

pub struct Projectile {
    pub damage: i32,
    pub ttl: f32,
    pub piercing: bool,
    pub speed: f32,
}

impl Projectile {
    pub fn new(damage: i32, ttl: f32, piercing: bool, speed: f32) -> Self {
        Projectile { damage, ttl, piercing, speed }
    }

    pub fn arrow() -> Projectile {
        data::get_projectile_info(ProjectileType::Arrow)
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
