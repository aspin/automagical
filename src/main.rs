mod animation;
mod asset_loader;
mod biome;
mod builder;
mod config;
mod construction;
mod controls;
mod cursor;
mod damage;
mod data;
mod enemy;
mod global_constants;
mod inventory;
mod projectile;
mod ui;
mod weapon;
mod world_map;
mod world_renderer;

use bevy::prelude::*;
use bevy_rapier3d::physics::RapierPhysicsPlugin;

fn main() {
    App::build()
        .init_resource::<world_map::WorldMap>()
        .add_resource(config::window_descriptor())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(asset_loader::AssetLoaderPlugin)
        .add_plugin(world_renderer::MapGeneratorPlugin)
        .add_plugin(inventory::InventoryPlugin)
        .add_plugin(ui::UiPlugin)
        .add_system(animation::animate.system())
        .add_system(builder::produce_projectiles.system())
        .add_system(projectile::expire_projectiles.system())
        .add_system(controls::control_builder.system())
        .add_system(cursor::update_cursor_position.system())
        .add_system(construction::place_object.system())
        .add_system(damage::examine_collisions.system())
        .add_system(enemy::move_enemies.system())
        .run();
}
