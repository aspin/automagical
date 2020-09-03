mod asset_loader;
mod map_generator;

use bevy::prelude::*;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(asset_loader::AssetLoaderPlugin)
        .add_plugin(map_generator::MapGeneratorPlugin)
        .run();
}

