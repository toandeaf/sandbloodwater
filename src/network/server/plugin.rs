use crate::network::client::handle_client_events;
use crate::network::server::infra::{initialize_server, read_from_event_queue};
use crate::network::server::system::{process_network_events, process_new_connection_events};
use crate::network::NewConnectionEvent;
use bevy::app::{App, Plugin, Startup};
use bevy::prelude::Main;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewConnectionEvent>()
            .add_systems(Startup, initialize_server)
            .add_systems(
                Main,
                (
                    read_from_event_queue,
                    handle_client_events,
                    process_network_events,
                    process_new_connection_events,
                ),
            );
    }
}
