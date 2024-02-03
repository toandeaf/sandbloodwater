use crate::common::SERVER_ADDRESS;
use bevy::prelude::*;

use crate::network_client::client::{dispatcher, receive_events, Client};
use crate::network_client::system::process_move_client;
use crate::network_client::HttpClient;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        let http_client_res = HttpClient::new(SERVER_ADDRESS);

        if let Ok(http_client) = http_client_res {
            app.insert_resource(Client(http_client))
                .add_systems(Main, (process_move_client, receive_events, dispatcher));
        }
    }
}
