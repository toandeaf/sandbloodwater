use std::io::{BufRead, BufReader, BufWriter, Error, Read, Write};
use std::net::TcpStream;

use bevy::prelude::{EventReader, EventWriter, Events, ResMut, Resource};

use crate::common::{EventWrapper, EOF};
use crate::player::{MovementEvent, PlayerCreateEvent};

#[derive(Resource)]
pub struct Client(pub HttpClient);

pub struct HttpClient {
    writer: BufWriter<TcpStream>,
    reader: BufReader<TcpStream>,
}

impl Client {
    pub fn new(addr: &str) -> Result<Client, Error> {
        let connection = TcpStream::connect(addr)?;
        connection.set_nonblocking(true).unwrap();

        let read_connection = connection.try_clone().unwrap();

        Ok(Client(HttpClient {
            reader: BufReader::new(read_connection),
            writer: BufWriter::new(connection),
        }))
    }

    pub fn send_event(&mut self, event: EventWrapper) {
        if let Ok(mut event_bytes) = serde_json::to_vec(&event) {
            event_bytes.push(EOF);

            let bytes_written = self.0.writer.write(&event_bytes).expect("FUCKED THE WRITE");

            if bytes_written == event_bytes.len() {
                self.0.writer.flush().unwrap();
            }

            println!("Sending event: {}", String::from_utf8(event_bytes).unwrap());
        }
    }

    pub fn receive_event(&mut self) -> Option<EventWrapper> {
        let mut buffer = vec![];

        if let Ok(bytes_read) = self.0.reader.read_until(EOF, &mut buffer) {
            println!("BYTES READ {}", bytes_read);

            if bytes_read == 0 {
                // End of stream
                return None;
            }

            // TODO - evaluate if I actually need this given the read_until above.
            if let Some(delimit_position) = buffer.iter().position(|&byte| byte == EOF) {
                let event_wrapper =
                    serde_json::from_slice::<EventWrapper>(&buffer[..delimit_position])
                        .expect("Failed to parse event wrapper.");

                // Remove the processed message from the buffer
                buffer.drain(..=delimit_position);
                return Some(event_wrapper);
            }
        };
        None
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
