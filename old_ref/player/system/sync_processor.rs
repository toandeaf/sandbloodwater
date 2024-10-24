use bevy::prelude::*;
use bevy::utils::Uuid;

use crate::player::system::PlayerMapping;
use crate::player::{CharacterMarker, CurrentDirection, PlayerCreateEvent, PlayerSyncEvent};

pub fn process_sync(
    mut character_query: Query<(&mut Transform, &mut CurrentDirection), With<CharacterMarker>>,
    mut event_reader: EventReader<PlayerSyncEvent>,
    mut player_create_writer: EventWriter<PlayerCreateEvent>,
    player_mapping: ResMut<PlayerMapping>,
) {
    for sync_event in event_reader.read() {
        let (character_uuid, new_position, new_direction) =
            (sync_event.0, sync_event.1, sync_event.2);

        if let Some(character_entity) = player_mapping.0.get::<Uuid>(&character_uuid) {
            let bundle_res = character_query.get_mut(*character_entity);

            if let Ok((mut transform, mut direction)) = bundle_res {
                transform.translation.x = new_position.x;
                transform.translation.y = new_position.y;
                direction.0 = new_direction;
            }
            return;
        }

        player_create_writer.send(PlayerCreateEvent(
            character_uuid,
            new_position,
            new_direction,
        ));
    }
}
