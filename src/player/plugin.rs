use bevy::prelude::{App, Plugin, Startup};

use crate::player::system::{add_players, iterate_through_players};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (add_players, iterate_through_players));
    }

    fn name(&self) -> &str {
        "player_plugin"
    }

    fn is_unique(&self) -> bool {
        true
    }
}
