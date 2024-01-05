use bevy::prelude::{App, Plugin};

use crate::camera::CameraPlugin;
use crate::item::ItemPlugin;
use crate::player::PlayerPlugin;
use crate::world::WorldPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WorldPlugin, CameraPlugin, PlayerPlugin, ItemPlugin));
    }
}
