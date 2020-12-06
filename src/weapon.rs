use crate::data;
use crate::projectile::ProjectileType;

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub enum WeaponType {
    MagicBow,
}

pub struct Weapon {
    pub size: f32,
    pub projectile_type: Option<ProjectileType>,
    pub projectile_count: u32,
    pub projectile_launch_speed: f32,
    pub projectile_spread: f32,
}

impl Weapon {
    pub fn new(
        size: f32,
        projectile_type: Option<ProjectileType>,
        projectile_count: u32,
        projectile_launch_speed: f32,
        projectile_spread: f32,
    ) -> Self {
        Weapon {
            size,
            projectile_type,
            projectile_count,
            projectile_launch_speed,
            projectile_spread,
        }
    }

    pub fn magic_bow() -> Weapon {
        data::get_weapon_info(WeaponType::MagicBow)
    }

    pub fn spread_range(&self) -> std::ops::Range<i32> {
        let lower_range = (self.projectile_count as f32 / 2.).floor() as i32;
        let upper_range = (self.projectile_count as f32 / 2.).ceil() as i32;
        (0 - lower_range)..(upper_range)
    }
}
