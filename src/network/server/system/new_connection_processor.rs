use crate::common::EventWrapper;
use crate::network::server::infra::{dispatch_all, SESSION_CLIENTS};
use crate::network::NewConnectionEvent;
use crate::player::{CharacterMarker, CurrentDirection, PlayerSyncEvent};
use bevy::prelude::{EventReader, Query, Transform, Vec3Swizzles};

pub fn process_new_connection_events(
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
