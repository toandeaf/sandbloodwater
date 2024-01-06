use crate::player::resource::create_player_resources;
use crate::player::system::{
    initialise_player, interact, move_player, move_reader, process_movement, MovementEvent,
};
use bevy::prelude::{App, Main, Plugin, Startup};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(create_player_resources())
            .add_event::<MovementEvent>()
            .add_systems(Startup, initialise_player)
            .add_systems(Main, (move_player, interact, move_reader, process_movement));
    }
}
