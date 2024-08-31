use bevy::prelude::App;
use bevy::DefaultPlugins;
use bevy_embedded_assets::EmbeddedAssetPlugin;

use sandbloodwater::game::GamePlugin;
use sandbloodwater::network::NetworkPlugin;

// TODO - huge fuckin rewrite
// I'm gonna need a way to A) use `MinimalPlugins` instead of Default - I need the server to run
// headless and not waste compute on stuff that clients will be taking care of (for now, although if
// I ever do want a server evaluating the actions it'll need to be modular enough to pull back in.
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EmbeddedAssetPlugin::default(),
            GamePlugin,
            NetworkPlugin,
        ))
        .run();
}
