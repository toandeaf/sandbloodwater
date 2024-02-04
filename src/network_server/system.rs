use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, RwLock};
use std::thread;

use bevy::prelude::EventWriter;
use bevy::utils::HashMap;
use lazy_static::lazy_static;

use crate::common::{EventWrapper, EOF, SERVER_ADDRESS};
use crate::network_server::resource::Server;

lazy_static! {
    static ref SESSION_CLIENTS: RwLock<HashMap<usize, TcpStream>> = RwLock::new(HashMap::new());
    static ref EVENT_QUEUE: RwLock<VecDeque<EventWrapper>> = RwLock::new(VecDeque::new());
}

pub fn initialize_server() {
    let server = Server::new(SERVER_ADDRESS).expect("Couldn't initialise server.");

    let (event_sender, event_receiver) = mpsc::channel::<EventWrapper>();

    thread::spawn(move || {
        for stream in server.0.incoming() {
            let sender = event_sender.clone();

            if let Ok(data) = stream {
                thread::spawn(move || handle_client_connection(data, sender));
            }
        }
    });

    thread::spawn(|| {
        for event in event_receiver {
            if let Ok(mut queue) = EVENT_QUEUE.write() {
                queue.push_front(event);
            }
        }
    });
}

pub fn handle_client_connection(client_stream: TcpStream, event_sender: Sender<EventWrapper>) {
    let mut owning_key: usize = 0;

    if let Ok(mut addresses) = SESSION_CLIENTS.write() {
        owning_key = addresses.len() + 1;
        addresses.insert(owning_key, client_stream.try_clone().unwrap());
    }

    let mut buffer = vec![];
    let mut reader = BufReader::new(client_stream);

    // TODO introduce some shared state that can be polled for event or state data.
    loop {
        if let Ok(bytes_read) = reader.read_until(EOF, &mut buffer) {
            if bytes_read == 0 {
                // End of stream
                break;
            }

            // TODO - evaluate if I actually need this given the read_until above.
            if let Some(delimit_position) = buffer.iter().position(|&byte| byte == EOF) {
                if let Ok(data) = SESSION_CLIENTS.read() {
                    // TODO is there a better broadcast implementation rather than iterating like this?
                    for (key, mut connection) in data.iter() {
                        if &owning_key != key {
                            connection.write_all(&buffer[..=delimit_position]).unwrap();
                            connection.flush().unwrap();
                        }
                    }
                }

                let event_wrapper =
                    serde_json::from_slice::<EventWrapper>(&buffer[..delimit_position])
                        .expect("Failed to parse event wrapper.");

                event_sender
                    .send(event_wrapper)
                    .expect("Failed to send to receiver.");

                // Remove the processed message from the buffer
                buffer.drain(..=delimit_position);
            }
        }
    }
}

pub fn read_from_event_queue(mut event_writer: EventWriter<EventWrapper>) {
    if let Ok(queue) = EVENT_QUEUE.read() {
        if queue.is_empty() {
            return;
        }
    }

    if let Ok(mut queue) = EVENT_QUEUE.write() {
        if let Some(event) = queue.pop_front() {
            event_writer.send(event);
        }
    };
}
