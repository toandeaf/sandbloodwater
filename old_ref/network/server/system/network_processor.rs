use crate::common::EventWrapper;
use crate::network::resource::NetworkWrapper;
use crate::network::server::infra::dispatch_all_except_origin;
use crate::network::NewConnectionEvent;
use crate::player::{MovementEvent, PlayerCreateEvent, PlayerSyncEvent};
use bevy::prelude::{EventReader, EventWriter};

pub fn process_network_events(
    mut event_reader: EventReader<NetworkWrapper>,
    mut new_connection_writer: EventWriter<NewConnectionEvent>,
    mut player_create_writer: EventWriter<PlayerCreateEvent>,
    mut player_sync_writer: EventWriter<PlayerSyncEvent>,
    mut movement_writer: EventWriter<MovementEvent>,
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
            EventWrapper::PlayerSync(event) => {
                player_sync_writer.send(*event);
            }
            EventWrapper::Movement(event) => {
                dispatch_all_except_origin(network_wrapper);
                movement_writer.send(*event);
            }
            // _ => { dispatch_all_except_origin(network_wrapper) },
            _ => {}
        }
    }
}
