use std::collections::VecDeque;
use std::sync::mpsc;
use std::sync::RwLock;
use std::thread;

use bevy::prelude::EventWriter;
use lazy_static::lazy_static;

use crate::common::SERVER_ADDRESS;
use crate::network::resource::NetworkWrapper;
use crate::network::server::infra::client_handler::handle_client_connection;
use crate::network::server::resource::Server;

lazy_static! {
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
