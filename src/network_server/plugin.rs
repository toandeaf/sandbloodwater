use std::collections::VecDeque;
use std::sync::{mpsc, RwLock};
use std::thread;

use bevy::app::{App, Plugin, Startup};
use bevy::prelude::{EventWriter, Main};
use lazy_static::lazy_static;

use crate::common::{EventWrapper, SERVER_ADDRESS};
use crate::network_client::event_handler;
use crate::network_server::process_connection;
use crate::network_server::server::HttpServer;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_server)
            .add_systems(Main, (read_from_queue, event_handler));
    }
}

lazy_static! {
    static ref EVENT_QUEUE: RwLock<VecDeque<EventWrapper>> = RwLock::new(VecDeque::new());
}

fn read_from_queue(mut event_writer: EventWriter<EventWrapper>) {
    if let Ok(queue) = EVENT_QUEUE.read() {
        if queue.is_empty() {
            return;
        }
    }
    if let Ok(mut queue) = EVENT_QUEUE.write() {
        if let Some(event) = queue.pop_front() {
            println!(
                "Processing server event {}",
                serde_json::to_string::<EventWrapper>(&event).unwrap()
            );
            event_writer.send(event);
        }
    }
}

fn initialize_server() {
    let server = HttpServer::new(SERVER_ADDRESS).unwrap();
    server.listener.set_nonblocking(true).unwrap();

    let (event_sender, event_receiver) = mpsc::channel::<EventWrapper>();

    thread::spawn(move || {
        for stream in server.listener.incoming() {
            let sender = event_sender.clone();

            if let Ok(data) = stream {
                thread::spawn(move || process_connection(data, sender));
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
