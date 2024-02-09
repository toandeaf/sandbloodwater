use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, RwLock};
use std::thread;

use crate::player::PlayerCreateEvent;
use crate::player::PlayerSyncEvent;
use bevy::prelude::{EventReader, EventWriter, Query, Transform, Vec3Swizzles, With};
use bevy::utils::HashMap;
use lazy_static::lazy_static;

use crate::common::{EventWrapper, EOF, SERVER_ADDRESS};
use crate::network::resource::NetworkWrapper;
use crate::network::server::resource::Server;
use crate::network::NewConnectionEvent;
use crate::player::{CharacterMarker, CurrentDirection, Player};

lazy_static! {
    static ref SESSION_CLIENTS: RwLock<HashMap<u8, TcpStream>> = RwLock::new(HashMap::new());
    static ref USERNAME_SESSION_MAP: RwLock<HashMap<String, u8>> = RwLock::new(HashMap::new());
    static ref EVENT_QUEUE: RwLock<VecDeque<NetworkWrapper>> = RwLock::new(VecDeque::new());
}

pub fn initialize_server() {
    let server = Server::new(SERVER_ADDRESS).expect("Couldn't initialise server.");

    // Initialising sender and receiver for our mpsc channel. Allows the different client connection
    // threads to post back to our event receiver loop.
    let (event_sender, event_receiver) = mpsc::channel::<NetworkWrapper>();

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

pub fn handle_client_connection(client_stream: TcpStream, event_sender: Sender<NetworkWrapper>) {
    let mut connection_id: u8 = 0;

    // Wait for connection event from client. We won't initialise the main event parsing loop
    // until we get our connection event, which signals that the client instance is good to go.
    loop {
        let client_event_opt = read_client_event_stream(&client_stream);

        if let Some(EventWrapper::NewConnection(event_data)) = client_event_opt {
            let client_username = event_data.0;

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

            // If connection ID hasn't properly initialized.
            if connection_id == 0 {
                return;
            }

            // New connection, let's collect the mapping.
            if let Ok(mut session_map) = USERNAME_SESSION_MAP.write() {
                session_map.insert(client_username.clone(), connection_id);
            }

            event_sender
                .send(NetworkWrapper(
                    connection_id,
                    EventWrapper::NewConnection(NewConnectionEvent(client_username)),
                ))
                .expect("Failed to send connection event.");

            break;
        }
    }

    if let Ok(mut addresses) = SESSION_CLIENTS.write() {
        if let Ok(cloned_stream) = client_stream.try_clone() {
            addresses.insert(connection_id, cloned_stream);
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

pub fn read_from_event_queue(mut event_writer: EventWriter<NetworkWrapper>) {
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

pub fn read_server_events(
    mut event_reader: EventReader<NetworkWrapper>,
    mut new_connection_writer: EventWriter<NewConnectionEvent>,
    mut player_create_writer: EventWriter<PlayerCreateEvent>,
) {
    for network_wrapper in event_reader.read() {
        let event_wrapper = &network_wrapper.1;

        match event_wrapper {
            EventWrapper::NewConnection(new_connection_event) => {
                new_connection_writer
                    .send(NewConnectionEvent(String::from(&new_connection_event.0)));
            }
            EventWrapper::PlayerCreate(event) => {
                player_create_writer.send(*event);
            }
            EventWrapper::PlayerSync(_) => {
                println!("THIS SHOULDNT BE HERE?");
            }
            _ => dispatch_all_except_origin(network_wrapper),
        }
    }
}

pub fn process_new_connections(
    mut event_reader: EventReader<NewConnectionEvent>,
    player_query: Query<(&Transform, &CharacterMarker, &CurrentDirection)>,
) {
    for _event in event_reader.read() {
        if let Ok(data) = SESSION_CLIENTS.read() {
            println!("There are {} connections", data.len());
        }

        for (transform, marker, direction) in player_query.into_iter() {
            let player_sync_event = EventWrapper::PlayerSync(PlayerSyncEvent(
                marker.0,
                transform.translation.xy(),
                direction.0,
            ));
            dispatch_all(&player_sync_event)
        }
    }
}

fn dispatch_all_except_origin(network_wrapper: &NetworkWrapper) {
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

                let mut dataaa = serde_json::to_vec(event_wrapper).unwrap();
                dataaa.push(EOF);
                connection.write_all(&dataaa).unwrap();

                connection.flush().unwrap();
            }
        }
    }
}

fn dispatch_all(event_wrapper: &EventWrapper) {
    if let Ok(data) = SESSION_CLIENTS.read() {
        // TODO is there a better broadcast implementation rather than iterating like this?

        for mut connection in data.values() {
            println!(
                "ALL - Sending event {}",
                serde_json::to_string::<EventWrapper>(event_wrapper).unwrap()
            );

            let mut dataaa = serde_json::to_vec(event_wrapper).unwrap();
            dataaa.push(EOF);
            connection.write_all(&dataaa).unwrap();

            connection.flush().unwrap();
        }
    }
}
