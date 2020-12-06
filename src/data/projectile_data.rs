use crate::projectile::{Projectile, ProjectileType};

fn arrow_projectile_info() -> Projectile {
    Projectile::new(12, 2., true, 300.)
}

pub fn get_projectile_info(projectile_type: ProjectileType) -> Projectile {
    match projectile_type {
        ProjectileType::Arrow => arrow_projectile_info(),
    }
}
