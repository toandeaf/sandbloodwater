use crate::player::PlayerSyncEvent;
use bevy::prelude::{App, Main, Plugin, Startup};
use bevy::reflect::Uuid;
use bevy::utils::hashbrown::HashMap;

use crate::player::resource::{create_player_resources, PlayerUuid};
use crate::player::system::{
    dispatch_position_change, drop, init_player_dispatcher, initialise_player, interact,
    move_player, process_direction_change, process_drop, process_interact, process_position_change,
    process_sync, DropEvent, InteractionEvent, MovementEvent, PlayerCreateEvent, PlayerMapping,
    PlayerTextureAtlas,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(create_player_resources())
            .init_resource::<PlayerTextureAtlas>()
            .insert_resource(PlayerUuid(Uuid::new_v4()))
            .add_event::<DropEvent>()
            .add_event::<InteractionEvent>()
            .add_event::<MovementEvent>()
            .add_event::<PlayerSyncEvent>()
            .add_event::<PlayerCreateEvent>()
            .add_systems(
                Main,
                (
                    process_position_change,
                    process_direction_change,
                    process_interact,
                    process_drop,
                    process_sync,
                ),
            )
            .insert_resource(PlayerMapping(HashMap::new()));

        // TODO probably need a better bundling than this.
        #[cfg(feature = "client")]
        {
            app.add_systems(Startup, initialise_player).add_systems(
                Main,
                (
                    move_player,
                    interact,
                    drop,
                    dispatch_position_change,
                    init_player_dispatcher,
                ),
            );
        }
    }
}
