use bevy::DefaultPlugins;
use bevy::prelude::App;
use bevy_embedded_assets::EmbeddedAssetPlugin;

use sandbloodwater::game::GamePlugin;
use sandbloodwater::network_client::ClientPlugin;

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
