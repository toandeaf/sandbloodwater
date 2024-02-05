use std::io::Error;
use std::net::TcpListener;

use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Server(pub TcpListener);

impl Server {
    pub fn new(addr: &str) -> Result<Server, Error> {
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;

        Ok(Server(listener))
    }
}
