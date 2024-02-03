use bevy::prelude::App;
use bevy::DefaultPlugins;
use bevy_embedded_assets::EmbeddedAssetPlugin;

use sandbloodwater::game::GamePlugin;
use sandbloodwater::network_client::ClientPlugin;

// This is pretty much just to test a second client integration.
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EmbeddedAssetPlugin::default(),
            GamePlugin,
            ClientPlugin,
        ))
        .run();
}
