mod player_run;
mod entity_run;
mod world_run;
mod event_processing;
mod state_update;
mod render;
mod entity_response;
mod world_response;
mod state_update_events;
mod events;
mod server;
mod client;
mod standalone;
mod remote_client;
mod remote_server;

use crate::standalone::StandalonePlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(StandalonePlugin)
        .run();
}

