use crate::game::GamePlugin;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_embedded_assets::EmbeddedAssetPlugin;

mod camera;
mod common;
mod game;
mod item;
mod network;
mod player;
mod world;

#[allow(clippy::type_complexity)]
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EmbeddedAssetPlugin::default(), GamePlugin))
        .run();
}
