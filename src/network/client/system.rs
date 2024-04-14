use std::env::args;

use crate::player::PlayerCreateEvent;
use bevy::prelude::{EventReader, EventWriter, Events, ResMut};

use crate::common::EventWrapper;
use crate::network::client::resource::Client;
use crate::network::NewConnectionEvent;
use crate::player::{MovementEvent, PlayerSyncEvent};

pub fn initialise_connection(mut client: ResMut<Client>) {
    if let Some(username) = args().nth(1) {
        client.send_event(EventWrapper::NewConnection(NewConnectionEvent(username)));
    } else {
        client.send_event(EventWrapper::NewConnection(NewConnectionEvent(
            String::from("Default"),
        )));
    }
}

pub fn fetch_events_from_server(
    mut client: ResMut<Client>,
    mut events: ResMut<Events<EventWrapper>>,
) {
    let received_events = client.receive_event();

    for event in received_events.into_iter() {
        events.send(event);
    }
}

pub fn handle_client_events(
    mut event_reader: EventReader<EventWrapper>,
    mut movement_event_writer: EventWriter<MovementEvent>,
    mut player_sync_writer: EventWriter<PlayerSyncEvent>,
    mut player_create_writer: EventWriter<PlayerCreateEvent>,
) {
    for event in event_reader.read() {
        match event {
            EventWrapper::Movement(event_data) => {
                movement_event_writer.send(*event_data);
            }
            EventWrapper::PlayerSync(event_data) => {
                player_sync_writer.send(*event_data);
            }
            EventWrapper::PlayerCreate(event_data) => {
                player_create_writer.send(*event_data);
            }
            EventWrapper::Test(_) => {}
            EventWrapper::NewConnection(_) => {}
            EventWrapper::Disconnect(_) => {}
        }
    }
}
