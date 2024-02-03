use bevy::prelude::*;

use crate::common::EventWrapper;
use crate::network_client::Client;
use crate::player::MovementEvent;

#[allow(clippy::type_complexity)]
pub fn process_move_client(
    mut event_reader: EventReader<MovementEvent>,
    mut client: ResMut<Client>,
) {
    for event in event_reader.read() {
        client.0.send_event(EventWrapper::Movement(*event));
    }
}
