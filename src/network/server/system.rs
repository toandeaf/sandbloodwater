use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, RwLock};
use std::thread;

use bevy::prelude::{EventReader, EventWriter};
use bevy::utils::{HashMap, Uuid};
use lazy_static::lazy_static;

use crate::common::{EventWrapper, EOF, SERVER_ADDRESS};
use crate::network::server::resource::Server;

lazy_static! {
    static ref SESSION_CLIENTS: RwLock<HashMap<Uuid, TcpStream>> = RwLock::new(HashMap::new());
    static ref EVENT_QUEUE: RwLock<VecDeque<EventWrapper>> = RwLock::new(VecDeque::new());
}

pub fn initialize_server() {
    let server = Server::new(SERVER_ADDRESS).expect("Couldn't initialise server.");

    // Initialising sender and receiver for our mpsc channel. Allows the different client connection
    // threads to post back to our event receiver loop.
    let (event_sender, event_receiver) = mpsc::channel::<EventWrapper>();

    // Spawning a new thread to listen for new incoming connections.
    thread::spawn(move || {
        for new_connection in server.0.incoming() {
            let sender = event_sender.clone();

            if let Ok(data) = new_connection {
                // Spawning a new thread to handle each new client connection.
                thread::spawn(move || handle_client_connection(data, sender));
            }
        }
    });

    // Spawning a new thread to handle the event receiver for the mpsc channel.
    thread::spawn(|| {
        for event in event_receiver {
            if let Ok(mut queue) = EVENT_QUEUE.write() {
                // For each new event, it'll push to the event queue, which is being read
                // from as part of the game loop.
                queue.push_front(event);
            }
        }
    });
}

pub fn handle_client_connection(client_stream: TcpStream, event_sender: Sender<EventWrapper>) {
    let mut connection_uuid: Uuid = Default::default();

    // Wait for connection event from client. We won't initialise the main event parsing loop
    // until we get our connection event, which signals that the client instance is good to go.
    loop {
        let client_event_opt = read_client_event_stream(&client_stream);

        if let Some(EventWrapper::NewConnection(event_data)) = client_event_opt {
            if let Ok(mut addresses) = SESSION_CLIENTS.write() {
                connection_uuid = event_data.0;
                addresses.insert(connection_uuid, client_stream.try_clone().unwrap());
            }

            event_sender
                .send(EventWrapper::NewConnection(event_data))
                .expect("Failed to connection main event to channel.");

            break;
        }
    }

    // Main client event loop. Reading events, sending events back to channel receiver.
    loop {
        let client_event_opt = read_client_event_stream(&client_stream);

        if let Some(client_event) = client_event_opt {
            match client_event {
                // Disconnect is the only event (until connection error handling is implemented),
                // that will break the game loop.
                EventWrapper::Disconnect(_) => {
                    break;
                }
                other_wrappers => event_sender
                    .send(other_wrappers)
                    .expect("Failed to send main event to channel."),
            }
        }
    }

    // If we're breaking out of the client event loop, we should probably discard the connection.
    if let Ok(mut addresses) = SESSION_CLIENTS.write() {
        addresses.remove(&connection_uuid);
    }
}

fn read_client_event_stream(client_stream: &TcpStream) -> Option<EventWrapper> {
    // TODO surely reinitialising this vec and reader every time is shit?
    let mut buffer = vec![];
    let mut reader = BufReader::new(client_stream);

    if let Ok(bytes_read) = reader.read_until(EOF, &mut buffer) {
        if bytes_read == 0 {
            // End of stream
            return None;
        }

        // TODO - evaluate if I actually need this given the read_until above.
        if let Some(delimit_position) = buffer.iter().position(|&byte| byte == EOF) {
            let event_wrapper = serde_json::from_slice::<EventWrapper>(&buffer[..delimit_position])
                .expect("Failed to parse event wrapper.");

            // Remove the processed message from the buffer
            buffer.drain(..=delimit_position);

            return Some(event_wrapper);
        }
    }

    None
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

pub fn read_server_events(mut event_reader: EventReader<EventWrapper>) {
    for event_wrapper in event_reader.read() {
        match event_wrapper {
            EventWrapper::Movement(event) => dispatch(event.0, event_wrapper),
            EventWrapper::NewConnection(_event) => {
                // TODO implement new connection events resulting in player sync events for everyone.
                // dispatch(event.0, EventWrapper)
            }
            EventWrapper::Test(_) => {}
            _ => {}
        }
    }
}

pub fn dispatch(uuid: Uuid, event_wrapper: &EventWrapper) {
    if let Ok(data) = SESSION_CLIENTS.read() {
        // TODO is there a better broadcast implementation rather than iterating like this?
        for (key, mut connection) in data.iter() {
            if !uuid.eq(key) {
                connection
                    .write_all(&serde_json::to_vec(&event_wrapper).unwrap())
                    .unwrap();
                connection.flush().unwrap();
            }
        }
    }
}
