use crate::player::entity::{create_character_entity, create_player_entity};
use crate::player::resource::PlayerUuid;
use crate::player::system::init::PLAYER_Z_INDEX;
use crate::player::system::{PlayerMapping, PlayerTextureAtlas};
use crate::player::PlayerSyncEvent;
use bevy::prelude::{Commands, EventReader, Res, ResMut, Vec3};

pub fn process_sync(
    mut commands: Commands,
    mut event_reader: EventReader<PlayerSyncEvent>,
    player_texture_atlas: ResMut<PlayerTextureAtlas>,
    mut player_mapping: ResMut<PlayerMapping>,
    player_uuid: Res<PlayerUuid>,
) {
    for event in event_reader.read() {
        let uuid = player_uuid.0;

        if let Some(entity) = player_mapping.0.remove(&uuid) {
            // If they already exist, despawn em. We're probably gonna make this a fixed value and
            // not a dynamically generated uuid so we can get some consistency across sessions.
            commands.entity(entity).despawn();
        }

        println!(
            "Player uuid is {} and event uuid is {}",
            player_uuid.0.to_string(),
            uuid.to_string()
        );
        let player_entity = if uuid == player_uuid.0 {
            commands
                .spawn(create_player_entity(
                    event.0,
                    player_texture_atlas.0.clone(),
                    Vec3::from((event.1, PLAYER_Z_INDEX)),
                    event.2,
                ))
                .id()
        } else {
            commands
                .spawn(create_character_entity(
                    event.0,
                    player_texture_atlas.0.clone(),
                    Vec3::from((event.1, PLAYER_Z_INDEX)),
                    event.2,
                ))
                .id()
        };

        player_mapping.0.insert(uuid, player_entity);
    }
}
