use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;

use crate::game::GamePlugin;

mod camera;
mod game;
mod item;
mod player;
mod world;

#[allow(clippy::type_complexity)]
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EmbeddedAssetPlugin::default(), GamePlugin))
        .run();
}
