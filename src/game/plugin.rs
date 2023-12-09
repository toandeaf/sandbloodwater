use crate::camera::CameraPlugin;
use crate::player::PlayerPlugin;
use bevy::prelude::{App, Plugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CameraPlugin, PlayerPlugin));
    }

    fn name(&self) -> &str {
        "game_plugin"
    }

    fn is_unique(&self) -> bool {
        true
    }
}
