use bevy::input::InputPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::time::TimePlugin;
use bevy_embedded_assets::EmbeddedAssetPlugin;

use crate::game::GamePlugin;

mod camera;
mod common;
mod game;
mod item;
mod network;
mod player;
mod world;

#[allow(clippy::type_complexity)]
fn main() {
    let mut app = App::new();

    app.add_plugins((EmbeddedAssetPlugin::default(), GamePlugin));
    app.add_plugins((LogPlugin::default(), TaskPoolPlugin::default(),
                     TypeRegistrationPlugin, FrameCountPlugin, TimePlugin,
                     TransformPlugin, HierarchyPlugin));

    #[cfg(feature = "client")]
    {
        app.add_plugins((InputPlugin, WindowPlugin::default()));
    }

    app.run();
}
