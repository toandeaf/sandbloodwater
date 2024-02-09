use bevy::prelude::{EventReader, EventWriter, Events, ResMut};
use bevy::utils::Uuid;

use crate::common::EventWrapper;
use crate::network::client::resource::Client;
use crate::network::NewConnectionEvent;
use crate::player::{MovementEvent, PlayerSyncEvent};

pub fn initialise_connection(mut client: ResMut<Client>) {
    client.send_event(EventWrapper::NewConnection(NewConnectionEvent(
        Uuid::new_v4(),
    )));
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
    mut player_event_writer: EventWriter<PlayerSyncEvent>,
) {
    for event in event_reader.read() {
        match event {
            EventWrapper::Movement(event_data) => movement_event_writer.send(*event_data),
            EventWrapper::PlayerSync(event_data) => player_event_writer.send(*event_data),
            EventWrapper::Test(_) => {}
            EventWrapper::NewConnection(_event) => {}
            EventWrapper::Disconnect(_) => {}
        }
    }
}
