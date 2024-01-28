use crate::camera::CameraPlugin;
use crate::item::ItemPlugin;
use crate::network::NetworkPlugin;
use crate::player::PlayerPlugin;
use crate::world::WorldPlugin;
use bevy::prelude::{App, Plugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            WorldPlugin,
            NetworkPlugin,
            CameraPlugin,
            PlayerPlugin,
            ItemPlugin,
        ));
    }
}
