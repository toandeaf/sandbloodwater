use std::io::{BufRead, BufReader, Error, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::RwLock;

use bevy::prelude::Resource;
use bevy::utils::HashMap;
use lazy_static::lazy_static;

use crate::common::{EventWrapper, EOF};

#[derive(Resource)]
pub struct Server(pub HttpServer);

pub struct HttpServer {
    pub listener: TcpListener,
}

impl HttpServer {
    pub fn new(addr: &str) -> Result<HttpServer, Error> {
        let listener = TcpListener::bind(addr)?;

        Ok(HttpServer { listener })
    }
}

lazy_static! {
    static ref SESSION_CLIENTS: RwLock<HashMap<usize, TcpStream>> = RwLock::new(HashMap::new());
}

pub fn process_connection(stream: TcpStream, sender: Sender<EventWrapper>) {
    let mut owning_key: usize = 0;

    if let Ok(mut addresses) = SESSION_CLIENTS.write() {
        owning_key = addresses.len() + 1;
        addresses.insert(owning_key, stream.try_clone().unwrap());
    }

    let mut buffer = vec![];
    let mut reader = BufReader::new(stream);

    // TODO introduce some shared state that can be polled for event or state data.
    loop {
        if let Ok(bytes_read) = reader.read_until(EOF, &mut buffer) {
            if bytes_read == 0 {
                // End of stream
                break;
            }

            // TODO - evaluate if I actually need this given the read_until above.
            if let Some(delimit_position) = buffer.iter().position(|&x| x == EOF) {
                let full_data_range = ..=delimit_position;
                let event_data_range = ..delimit_position;

                if let Ok(data) = SESSION_CLIENTS.read() {
                    // TODO is there a better broadcast implementation rather than iterating like this?
                    for (key, mut connection) in data.iter() {
                        if &owning_key != key {
                            connection
                                .write_all(&buffer[event_data_range])
                                .expect("Issue broadcasting to client");
                        }
                    }
                }

                let event_wrapper =
                    serde_json::from_slice::<EventWrapper>(&buffer[event_data_range])
                        .expect("Failed to parse event wrapper.");

                sender
                    .send(event_wrapper)
                    .expect("Failed to send to receiver.");

                // Remove the processed message from the buffer
                buffer.drain(full_data_range);
            }
        }
    }
}
