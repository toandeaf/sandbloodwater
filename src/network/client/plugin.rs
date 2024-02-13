use bevy::prelude::*;

use crate::common::SERVER_ADDRESS;
use crate::network::client::handle_client_events;
use crate::network::client::resource::Client;
use crate::network::client::system::{initialise_connection, fetch_events_from_server};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        let client_res = Client::new(SERVER_ADDRESS);

        if let Ok(http_client) = client_res {
            app.insert_resource(http_client)
                .add_systems(PreStartup, initialise_connection)
                .add_systems(Main, (handle_client_events, fetch_events_from_server));
        }
    }
}
