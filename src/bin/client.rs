use std::io::{Error, Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

use bevy::prelude::Event;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Event)]
pub enum EventId {
    Test(String),
    More(String),
}

struct HttpClient {
    connection: TcpStream,
    buffer: [u8; 512],
}

impl HttpClient {
    fn new(addr: &str) -> Result<HttpClient, Error> {
        let connection = TcpStream::connect(addr)?;
        Ok(HttpClient {
            connection,
            buffer: [0; 512],
        })
    }

    fn send_event(&mut self, event: EventId) {
        if let Ok(event_bytes) = serde_json::to_vec(&event) {
            let _write_result = self.connection.write(event_bytes.as_slice());
        }
    }

    fn receive_event(&mut self) {
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

fn main() {
    let client_res = HttpClient::new("127.0.0.1:7878");

    if let Ok(mut client) = client_res {
        loop {
            client.send_event(EventId::Test(String::from("Test string content")));
            client.receive_event();
        }
    }
}
