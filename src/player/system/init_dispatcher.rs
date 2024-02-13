use crate::common::EventWrapper;
use crate::network::Client;
use crate::player::system::PlayerCreateEvent;
use bevy::prelude::{EventReader, ResMut};

pub fn dispatch_init(
    mut event_reader: EventReader<PlayerCreateEvent>,
    mut client: ResMut<Client>,
) {
    for event in event_reader.read() {
        client.send_event(EventWrapper::PlayerCreate(*event));
    }
}
