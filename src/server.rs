use crate::entity_response::EntityResponsePlugin;
use crate::entity_run::EntityRunPlugin;
use crate::event_processing::EventProcessingPlugin;
use crate::events::EventsPlugin;
use crate::player_run::PlayerRunPlugin;
use crate::remote_server::RemoteServerPlugin;
use crate::state_update::StateUpdateSystemsPlugin;
use crate::state_update_events::StateUpdateEventsPlugin;
use crate::world_response::WorldResponsePlugin;
use crate::world_run::WorldRunSystemsPlugin;
use bevy::prelude::*;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((EventsPlugin, EventProcessingPlugin))
            .add_plugins((StateUpdateEventsPlugin, StateUpdateSystemsPlugin))
            .add_plugins((PlayerRunPlugin, EntityRunPlugin, WorldRunSystemsPlugin))
            .add_plugins((EntityResponsePlugin, WorldResponsePlugin))
            .add_plugins(RemoteServerPlugin)
            .add_systems(Startup, server_system);
    }
}

fn server_system() {}
