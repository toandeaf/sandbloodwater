use bevy::prelude::{Commands, EventReader, Res, ResMut, Vec3};

use crate::player::entity::{create_character_entity, create_player_entity};
use crate::player::PlayerCreateEvent;
use crate::player::resource::PlayerUuid;
use crate::player::system::EntityMapping;
use crate::player::system::init::{PLAYER_Z_INDEX, PlayerTextureAtlas};

pub fn process_init(
    mut commands: Commands,
    mut event_reader: EventReader<PlayerCreateEvent>,
    player_texture_atlas: Res<PlayerTextureAtlas>,
    mut player_mapping: ResMut<EntityMapping>,
    player_uuid: Res<PlayerUuid>,
) {
    for event in event_reader.read() {
        let event_uuid = event.0;
        let player_uuid = player_uuid.0;

        // if let Some(entity) = player_mapping.0.remove(&uuid) {
        //     // If they already exist, despawn em. We're probably gonna make this a fixed value and
        //     // not a dynamically generated uuid so we can get some consistency across sessions.
        //     commands.entity(entity).despawn();
        // }

        let args = PlayerTextureAtlas(
            player_texture_atlas.0.clone(),
            player_texture_atlas.1.clone(),
        );

        let player_entity = if event_uuid.eq(&player_uuid) {
            commands
                .spawn(create_player_entity(
                    event.0,
                    args,
                    Vec3::from((event.1, PLAYER_Z_INDEX)),
                    event.2,
                ))
                .id()
        } else {
            commands
                .spawn(create_character_entity(
                    event.0,
                    args,
                    Vec3::from((event.1, PLAYER_Z_INDEX)),
                    event.2,
                ))
                .id()
        };

        player_mapping.0.insert(event_uuid, player_entity);
    }
}
