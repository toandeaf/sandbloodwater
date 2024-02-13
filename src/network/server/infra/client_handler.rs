use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::mpsc::Sender;
use std::sync::RwLock;

use lazy_static::lazy_static;

use crate::common::{EventWrapper, EOF};
use crate::network::resource::NetworkWrapper;
use crate::network::NewConnectionEvent;

lazy_static! {
    pub static ref USERNAME_SESSION_MAP: RwLock<HashMap<String, u8>> = RwLock::new(HashMap::new());
    pub static ref SESSION_CLIENTS: RwLock<HashMap<u8, TcpStream>> = RwLock::new(HashMap::new());
}

pub fn handle_client_connection(client_stream: TcpStream, event_sender: Sender<NetworkWrapper>) {
    let connection_id: u8;

    // Wait for connection event from client. We won't initialise the main event parsing loop
    // until we get our connection event, which signals that the client instance is good to go.
    loop {
        let client_event_opt = read_client_event_stream(&client_stream);

        if let Some(EventWrapper::NewConnection(new_connection_event_data)) = client_event_opt {
            connection_id = process_new_connection_event(
                new_connection_event_data,
                &client_stream,
                &event_sender,
            );
            if connection_id == 0 {
                return;
            }
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
                    .send(NetworkWrapper(connection_id, other_wrappers))
                    .expect("Failed to send main event to channel."),
            }
        }
    }

    // If we're breaking out of the client event loop, we should probably discard the connection.
    if let Ok(mut addresses) = SESSION_CLIENTS.write() {
        addresses.remove(&connection_id);
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

fn process_new_connection_event(
    new_connection_event: NewConnectionEvent,
    client_stream: &TcpStream,
    event_sender: &Sender<NetworkWrapper>,
) -> u8 {
    let client_username = new_connection_event.0;
    let mut connection_id: u8 = 0;

    // TODO refactor this block so it looks less shit.
    if let Ok(session_map) = USERNAME_SESSION_MAP.read() {
        if let Some(con_id) = session_map.get::<String>(&client_username) {
            connection_id = *con_id;
        } else {
            let mut values: Vec<u8> = session_map.values().cloned().collect();
            values.sort();

            if let Some(last_value) = values.last() {
                connection_id = last_value + 1;
            } else {
                connection_id = 1;
            }
        }
    }

    // New connection, let's collect the mapping.
    if let Ok(mut session_map) = USERNAME_SESSION_MAP.write() {
        session_map.insert(client_username.clone(), connection_id);
    }

    if let Ok(mut addresses) = SESSION_CLIENTS.write() {
        if let Ok(cloned_stream) = client_stream.try_clone() {
            addresses.insert(connection_id, cloned_stream);
        }
    }

    event_sender
        .send(NetworkWrapper(
            connection_id,
            EventWrapper::NewConnection(NewConnectionEvent(client_username)),
        ))
        .expect("Failed to send connection event.");

    connection_id
}

pub fn dispatch_all_except_origin(network_wrapper: &NetworkWrapper) {
    let origin_client_key = network_wrapper.0;
    let event_wrapper = &network_wrapper.1;

    if let Ok(data) = SESSION_CLIENTS.read() {
        // TODO is there a better broadcast implementation rather than iterating like this?

        for (other_client_key, mut connection) in data.iter() {
            if !origin_client_key.eq(other_client_key) {
                // TODO do we need our EOF guy in here too?
                println!(
                    "SELECTED - Sending event {}",
                    serde_json::to_string::<EventWrapper>(event_wrapper).unwrap()
                );

                let mut event_data = serde_json::to_vec(event_wrapper).unwrap();
                event_data.push(EOF);
                connection.write_all(&event_data).unwrap();

                connection.flush().unwrap();
            }
        }
    }
}

pub fn dispatch_all(event_wrapper: &EventWrapper) {
    if let Ok(data) = SESSION_CLIENTS.read() {
        for mut connection in data.values() {
            println!(
                "ALL - Sending event {}",
                serde_json::to_string::<EventWrapper>(event_wrapper).unwrap()
            );

            let mut event_data = serde_json::to_vec(event_wrapper).unwrap();
            event_data.push(EOF);

            connection.write_all(&event_data).unwrap();

            connection.flush().unwrap();
        }
    }
}
