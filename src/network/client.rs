use std::io::{Error, Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

use bevy::prelude::{ResMut, Resource};

use crate::common::EventId;

#[derive(Resource)]
pub struct Client(pub HttpClient);

pub struct HttpClient {
    connection: TcpStream,
    buffer: [u8; 512],
}

const EOF: u8 = 0x03;

impl HttpClient {
    pub fn new(addr: &str) -> Result<HttpClient, Error> {
        let connection = TcpStream::connect(addr)?;
        Ok(HttpClient {
            connection,
            buffer: [0; 512],
        })
    }

    pub fn send_event(&mut self, event: EventId) {
        if let Ok(mut event_bytes) = serde_json::to_vec(&event) {
            event_bytes.push(EOF);
            let _write_result = self.connection.write(&event_bytes);
            println!("Sending event: {}", String::from_utf8(event_bytes).unwrap());
        }
    }

    pub fn receive_event(&mut self) {
        match self.connection.read(&mut self.buffer) {
            Ok(size) => {
                let received = from_utf8(&self.buffer[..size]).unwrap();
                println!("Received from server: {}", received);
            }
            Err(e) => {
                println!("Failed to receive data: {}", e);
            }
        }
    }
}

pub fn test_connection(mut client: ResMut<Client>) {
    client
        .0
        .send_event(EventId::Test(String::from("Connection Request")));
}
