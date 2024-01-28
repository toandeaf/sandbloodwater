use bevy::prelude::App;
use bevy::DefaultPlugins;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use sandbloodwater::game::GamePlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EmbeddedAssetPlugin::default(), GamePlugin))
        .run();
}
