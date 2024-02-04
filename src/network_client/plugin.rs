use bevy::prelude::*;

use crate::common::SERVER_ADDRESS;
use crate::network_client::client::{event_handler, receive_events, Client};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        let client_res = Client::new(SERVER_ADDRESS);

        if let Ok(http_client) = client_res {
            app.insert_resource(http_client)
                .add_systems(Main, (event_handler, receive_events));
        }
    }
}
