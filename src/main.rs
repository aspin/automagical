mod asset_loader;
mod map_generator;
mod builder;
mod controls;
mod world_map;
mod coordinate;

use bevy::prelude::*;

fn main() {
    App::build()
        .init_resource::<world_map::WorldMap>()
        .add_default_plugins()
        .add_plugin(asset_loader::AssetLoaderPlugin)
        .add_plugin(map_generator::MapGeneratorPlugin)
        .add_system(builder::animate.system())
        .add_system(controls::control_builder.system())
        .run();
}

