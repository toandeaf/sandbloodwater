use std::thread;

use crate::common::SERVER_ADDRESS;
use bevy::app::{App, Plugin, Startup};

use crate::network_server::process_connection;
use crate::network_server::server::HttpServer;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_server);
    }
}

fn initialize_server() {
    let server = HttpServer::new(SERVER_ADDRESS).unwrap();
    server.listener.set_nonblocking(true).unwrap();

    thread::spawn(move || {
        for stream in server.listener.incoming() {
            match stream {
                Ok(data) => {
                    thread::spawn(|| process_connection(data));
                }
                Err(_) => {
                    // TODO error handling.
                }
            }
        }
    });
}
