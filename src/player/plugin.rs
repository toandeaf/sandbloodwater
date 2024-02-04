use crate::player::PlayerCreateEvent;
use bevy::prelude::{App, Main, Plugin, Startup};
use bevy::utils::hashbrown::HashMap;

use crate::player::resource::create_player_resources;
use crate::player::system::{
    drop, initialise_player, interact, move_player, process_direction_change, process_drop,
    process_init, process_interact, process_position_change, DropEvent, InteractionEvent,
    MovementEvent, PlayerMapping, PlayerTextureAtlas,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(create_player_resources())
            .init_resource::<PlayerTextureAtlas>()
            .add_event::<MovementEvent>()
            .add_event::<InteractionEvent>()
            .add_event::<PlayerCreateEvent>()
            .add_event::<DropEvent>()
            .insert_resource(PlayerMapping(HashMap::new()))
            .add_systems(
                Main,
                (
                    process_position_change,
                    process_direction_change,
                    process_interact,
                    process_drop,
                    process_init,
                ),
            );

        // TODO probably need a better bundling than this.
        #[cfg(feature = "client")]
        {
            app.add_systems(Startup, initialise_player)
                .add_systems(Main, (move_player, interact, drop));
        }
    }
}
