use crate::animation::UnitType;
use bevy_rapier3d::rapier::geometry::ColliderBuilder;

fn wizard_collision_data() -> ColliderBuilder {
    ColliderBuilder::cuboid(5., 7., 8.)
}

fn enemy_collision_data() -> ColliderBuilder {
    ColliderBuilder::cuboid(5., 7., 8.)
}

pub fn get_collision_data(unit_type: UnitType) -> ColliderBuilder {
    match unit_type {
        UnitType::Wizard => wizard_collision_data(),
        UnitType::Enemy => enemy_collision_data(),
    }
}
