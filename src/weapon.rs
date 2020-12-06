use crate::projectile::ProjectileType;

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub enum WeaponType {
    MagicBow
}

pub struct WeaponInfo {
    pub projectile_type: Option<ProjectileType>,
    pub projectile_count: u32,
    pub projectile_launch_speed: f32,
    pub projectile_spread: f32,

}

impl WeaponInfo {
    pub fn new(
        projectile_type: Option<ProjectileType>,
        projectile_count: u32,
        projectile_launch_speed: f32,
        projectile_spread: f32
    ) -> Self {
        WeaponInfo { projectile_type, projectile_count, projectile_launch_speed, projectile_spread }
    }
}
