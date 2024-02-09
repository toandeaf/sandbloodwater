use bevy::prelude::{EventReader, ResMut};

use crate::common::EventWrapper;
use crate::network::Client;
use crate::player::system::r#move::MovementEvent;

#[allow(clippy::type_complexity)]
pub fn dispatch_position_change(
    mut event_reader: EventReader<MovementEvent>,
    mut client: ResMut<Client>,
) {
    for event in event_reader.read() {
        client.send_event(EventWrapper::Movement(*event));
    }
}
