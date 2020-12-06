use bevy::prelude::*;
use crate::data;

pub const ARROW_SPEED: f32 = 400.;
pub const ARROW_OFFSET: f32 = 3.;
pub const ARROW_SPREAD: f32 = 5.;

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub enum ProjectileType {
    Arrow
}

pub struct ProjectileInfo {
    pub damage: i32,
    pub lifetime: f32,
    pub piercing: bool,
    pub speed: f32,
}

impl ProjectileInfo {
    pub fn new(damage: i32, lifetime: f32, piercing: bool, speed: f32) -> Self {
        ProjectileInfo { damage, lifetime, piercing, speed }
    }

    pub fn to_projectile(&self) -> Projectile {
        Projectile::new(self.damage, self.lifetime, self.piercing)
    }
}

pub struct Projectile {
    pub damage: i32,
    pub ttl: f32,
    pub piercing: bool,
}

impl Projectile {
    pub fn new(damage: i32, ttl: f32, piercing: bool) -> Self {
        Projectile { damage, ttl, piercing }
    }

    pub fn arrow() -> Projectile {
        data::get_projectile_info(ProjectileType::Arrow).to_projectile()
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
