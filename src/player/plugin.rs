use bevy::prelude::{App, Main, Plugin, Startup};

use crate::player::resource::create_player_resources;
use crate::player::system::{
    drop, initialise_player, interact, move_player, process_direction_change, process_drop,
    process_interact, process_position_change, DropEvent, InteractionEvent, MovementEvent,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(create_player_resources())
            .add_event::<MovementEvent>()
            .add_event::<InteractionEvent>()
            .add_event::<DropEvent>()
            .add_systems(Startup, initialise_player)
            .add_systems(
                Main,
                (
                    process_position_change,
                    process_direction_change,
                    process_interact,
                    process_drop,
                ),
            );

        // TODO probably need a better bundling than this.
        #[cfg(feature = "client")]
        {
            app.add_systems(Main, (move_player, interact, drop));
        }
    }
}
