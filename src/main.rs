mod asset_loader;
mod world_renderer;
mod builder;
mod controls;
mod world_map;
mod coordinate;
mod projectile;
mod construction;

use bevy::prelude::*;
use bevy_rapier3d::physics::RapierPhysicsPlugin;

fn main() {
    App::build()
        .init_resource::<world_map::WorldMap>()
        .add_default_plugins()
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(asset_loader::AssetLoaderPlugin)
        .add_plugin(world_renderer::MapGeneratorPlugin)
        .add_system(builder::animate.system())
        .add_system(builder::produce_projectiles.system())
        .add_system(projectile::expire_projectiles.system())
        .add_system(controls::control_builder.system())
        .add_system(construction::update_cursor_position.system())
        .add_system(construction::place_object.system())
        .run();
}

