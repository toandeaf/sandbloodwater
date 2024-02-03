use std::io::{BufRead, BufReader, Error, Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

use bevy::prelude::{EventReader, EventWriter, Events, ResMut, Resource};

use crate::common::EventId;
use crate::player::MovementEvent;

#[derive(Resource)]
pub struct Client(pub HttpClient);

pub struct HttpClient {
    connection: TcpStream,
    reader: BufReader<TcpStream>,
    buffer: [u8; 512],
}

const EOF: u8 = 0x03;

impl HttpClient {
    pub fn new(addr: &str) -> Result<HttpClient, Error> {
        let connection = TcpStream::connect(addr)?;
        connection.set_nonblocking(true).unwrap();

        Ok(HttpClient {
            connection: connection.try_clone().unwrap(),
            reader: BufReader::new(connection),
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

    pub fn receive_event(&mut self) -> Option<EventId> {
        if let Ok(bytes_read) = self.connection.read(&mut self.buffer) {
            if bytes_read == 0 {
                return None;
            }

            // TODO need to delimit here too.

            let parsed_event_id =
                serde_json::from_slice::<EventId>(&self.buffer[..bytes_read]).unwrap();
            return Some(parsed_event_id);
        }
        None
    }
}

pub fn receive_events(mut client: ResMut<Client>, mut events: ResMut<Events<EventId>>) {
    let event_opt = client.0.receive_event();

    if let Some(event) = event_opt {
        events.send(event);
    }
}

pub fn dispatcher(
    mut event_reader: EventReader<EventId>,
    mut movement_event_writer: EventWriter<MovementEvent>,
) {
    for event in event_reader.read() {
        match event {
            EventId::Test(_) => {}
            EventId::More(_) => {}
            EventId::Movement(event) => movement_event_writer.send(*event),
        }
    }
}
