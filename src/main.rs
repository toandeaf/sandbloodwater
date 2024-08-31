use bevy::app::RunFixedMainLoop;
use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EmbeddedAssetPlugin::default(), GamePlugin))
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Main, testU);
        app.add_systems(PreStartup, testU);
        app.add_systems(Startup, testU);
        app.add_systems(PostStartup, testU);
        app.add_systems(First, testU);
        app.add_systems(PreUpdate, testU);
        app.add_systems(RunFixedMainLoop, testU);
        app.add_systems(FixedFirst, testU);
        app.add_systems(PostStartup, testU);

    }
}

fn testU() {}


