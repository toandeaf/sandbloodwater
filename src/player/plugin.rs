use bevy::prelude::{App, Main, Plugin, Startup};

use crate::player::system::{initialise_player, move_player};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialise_player)
            .add_systems(Main, move_player);
    }

    fn name(&self) -> &str {
        "player_plugin"
    }

    fn is_unique(&self) -> bool {
        true
    }
}
