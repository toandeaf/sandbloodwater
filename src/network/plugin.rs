use crate::network::client::{test_connection, Client};
use crate::network::HttpClient;
use bevy::prelude::*;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        let http_client_res = HttpClient::new("127.0.0.1:7878");

        if let Ok(http_client) = http_client_res {
            app.insert_resource(Client(http_client));
        }

        app.add_systems(Startup, test_connection);
    }
}
