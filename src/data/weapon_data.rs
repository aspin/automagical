use crate::weapon::{WeaponType, WeaponInfo};
use crate::projectile::ProjectileType;

fn magic_bow_info() -> WeaponInfo {
    WeaponInfo::new(Some(ProjectileType::Arrow), 3, 100., 5.)
}

pub fn get_weapon_info(weapon_type: WeaponType) -> WeaponInfo {
    match weapon_type {
        WeaponType::MagicBow => magic_bow_info()
    }
}
