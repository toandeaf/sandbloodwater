use bevy::prelude::*;

use crate::common::EventWrapper;
use crate::network_client::Client;
use crate::player::MovementEvent;

// TODO remove this.
// Once I decouple the "client-side" behaviour from the server-processing, we'll be able to reliably
// initialise the client and put the event push in the actual system function alongside the event emission.
#[allow(clippy::type_complexity)]
pub fn process_move_client(
    mut event_reader: EventReader<MovementEvent>,
    mut client: ResMut<Client>,
) {
    for event in event_reader.read() {
        // client.send_event(EventWrapper::Movement(*event));
    }
}
