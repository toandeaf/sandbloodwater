use crate::network::client::event_handler;
use crate::network::server::system::{initialize_server, read_from_event_queue};
use bevy::app::{App, Plugin, Startup};
use bevy::prelude::Main;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_server)
            .add_systems(Main, (read_from_event_queue, event_handler));
    }
}
