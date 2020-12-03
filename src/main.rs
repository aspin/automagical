mod asset_loader;
mod builder;
mod construction;
mod controls;
mod coordinate;
mod damage;
mod data;
mod enemy;
mod projectile;
mod world_map;
mod world_renderer;
mod animation;
mod biome;
mod global_constants;

use bevy::prelude::*;
use bevy_rapier3d::physics::RapierPhysicsPlugin;

fn main() {
    App::build()
        .init_resource::<world_map::WorldMap>()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(asset_loader::AssetLoaderPlugin)
        .add_plugin(world_renderer::MapGeneratorPlugin)
        .add_system(animation::animate.system())
        .add_system(builder::produce_projectiles.system())
        .add_system(projectile::expire_projectiles.system())
        .add_system(controls::control_builder.system())
        .add_system(construction::update_cursor_position.system())
        .add_system(construction::place_object.system())
        .add_system(damage::examine_collisions.system())
        .add_system(enemy::move_enemies.system())
        .run();
}
