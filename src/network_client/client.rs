use std::io::{Error, Read, Write};
use std::net::TcpStream;

use bevy::prelude::{EventReader, EventWriter, Events, ResMut, Resource};

use crate::common::EventWrapper;
use crate::player::MovementEvent;

#[derive(Resource)]
pub struct Client(pub HttpClient);

pub struct HttpClient {
    connection: TcpStream,
    // reader: BufReader<TcpStream>,
    buffer: [u8; 512],
}

const EOF: u8 = 0x03;

impl HttpClient {
    pub fn new(addr: &str) -> Result<HttpClient, Error> {
        let connection = TcpStream::connect(addr)?;
        connection.set_nonblocking(true).unwrap();

        Ok(HttpClient {
            connection,
            // reader: BufReader::new(connection),
            buffer: [0; 512],
        })
    }

    pub fn send_event(&mut self, event: EventWrapper) {
        if let Ok(mut event_bytes) = serde_json::to_vec(&event) {
            event_bytes.push(EOF);
            let _write_result = self.connection.write(&event_bytes);

            println!("Sending event: {}", String::from_utf8(event_bytes).unwrap());
        }
    }

    pub fn receive_event(&mut self) -> Vec<EventWrapper> {
        let mut events = vec![];
        if let Ok(bytes_read) = self.connection.read(&mut self.buffer) {
            if bytes_read == 0 {
                return events;
            }

            let break_point = self.buffer.iter().position(|e| e == &0x03);

            if let Some(position) = break_point {
                let parsed_event_id =
                    serde_json::from_slice::<EventWrapper>(&self.buffer[..position]);

                if let Ok(event_id) = parsed_event_id {
                    events.push(event_id);
                }
            }

            // TODO need to delimit here too.
        }
        events
    }
}

pub fn receive_events(mut client: ResMut<Client>, mut events: ResMut<Events<EventWrapper>>) {
    let received_events = client.0.receive_event();

    for event in received_events.into_iter() {
        events.send(event);
    }
}

pub fn event_handler(
    mut event_reader: EventReader<EventWrapper>,
    mut movement_event_writer: EventWriter<MovementEvent>,
) {
    for event in event_reader.read() {
        if let EventWrapper::Movement(event) = event {
            movement_event_writer.send(*event)
        }
    }
}
