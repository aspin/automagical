use crate::projectile::{ProjectileInfo, ProjectileType};

fn arrow_projectile_info() -> ProjectileInfo {
    ProjectileInfo::new(12, 2., true, 300.)
}

pub fn get_projectile_info(projectile_type: ProjectileType) -> ProjectileInfo {
    match projectile_type {
        ProjectileType::Arrow => arrow_projectile_info()
    }
}