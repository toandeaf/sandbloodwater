use std::io::{Error, Read, Write};
use std::net::TcpStream;

use bevy::prelude::{EventReader, EventWriter, Events, ResMut, Resource};

use crate::common::{EventWrapper, EOF};
use crate::player::{MovementEvent, PlayerCreateEvent};

#[derive(Resource)]
pub struct Client(pub HttpClient);

pub struct HttpClient {
    connection: TcpStream,
    buffer: [u8; 512],
}

impl Client {
    pub fn new(addr: &str) -> Result<Client, Error> {
        let connection = TcpStream::connect(addr)?;
        connection.set_nonblocking(true).unwrap();

        Ok(Client(HttpClient {
            connection,
            buffer: [0; 512],
        }))
    }

    pub fn send_event(&mut self, event: EventWrapper) {
        if let Ok(mut event_bytes) = serde_json::to_vec(&event) {
            event_bytes.push(EOF);
            let _write_result = self.0.connection.write(&event_bytes);

            println!("Sending event: {}", String::from_utf8(event_bytes).unwrap());
        }
    }

    pub fn receive_event(&mut self) -> Vec<EventWrapper> {
        let mut events = vec![];

        if let Ok(bytes_read) = self.0.connection.read(&mut self.0.buffer) {
            if bytes_read == 0 {
                return events;
            }

            let break_point = self.0.buffer.iter().position(|e| e == &EOF);

            if let Some(position) = break_point {
                let parsed_event_id =
                    serde_json::from_slice::<EventWrapper>(&self.0.buffer[..position]);

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
    let received_events = client.receive_event();

    for event in received_events.into_iter() {
        events.send(event);
    }
}

pub fn event_handler(
    mut event_reader: EventReader<EventWrapper>,
    mut movement_event_writer: EventWriter<MovementEvent>,
    mut player_event_writer: EventWriter<PlayerCreateEvent>,
) {
    for event in event_reader.read() {
        match event {
            EventWrapper::Movement(event_data) => movement_event_writer.send(*event_data),
            EventWrapper::PlayerCreate(event_data) => player_event_writer.send(*event_data),
            EventWrapper::Test(_) => {}
        }
    }
}
