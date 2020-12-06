use crate::weapon::{WeaponType, Weapon};
use crate::projectile::ProjectileType;

fn magic_bow_info() -> Weapon {
    Weapon::new(3., Some(ProjectileType::Arrow), 3, 100., 5.)
}

pub fn get_weapon_info(weapon_type: WeaponType) -> Weapon {
    match weapon_type {
        WeaponType::MagicBow => magic_bow_info()
    }
}
