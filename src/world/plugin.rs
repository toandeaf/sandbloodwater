use bevy::app::App;
use bevy::prelude::{Plugin, Startup};

use crate::world::system::initialise_map;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialise_map);
    }
}
