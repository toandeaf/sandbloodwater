use bevy::prelude::*;
use local_ip_address::local_ip;

use crate::common::GAME_PORT;
use crate::network::client::handle_client_events;
use crate::network::client::resource::Client;
use crate::network::client::system::{fetch_events_from_server, initialise_connection};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        // TODO read an override in from args? This will work for now.
        let local_ip = local_ip().unwrap();

        let formatted_local_ip = local_ip.to_string() + GAME_PORT;

        let client_res = Client::new(formatted_local_ip);

        if let Ok(http_client) = client_res {
            app.insert_resource(http_client)
                .add_systems(PreStartup, initialise_connection)
                .add_systems(Main, (handle_client_events, fetch_events_from_server));
        }
    }
}
