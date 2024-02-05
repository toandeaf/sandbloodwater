use bevy::prelude::{Commands, EventReader, ResMut, Vec3};

use crate::player::entity::create_character_entity;
use crate::player::system::init::{PlayerCreateEvent, PlayerTextureAtlas, PLAYER_Z_INDEX};
use crate::player::system::PlayerMapping;

pub fn process_init(
    mut commands: Commands,
    mut event_reader: EventReader<PlayerCreateEvent>,
    player_texture_atlas: ResMut<PlayerTextureAtlas>,
    mut player_mapping: ResMut<PlayerMapping>,
) {
    for event in event_reader.read() {
        let entity = commands
            .spawn(create_character_entity(
                event.0,
                player_texture_atlas.0.clone(),
                Vec3::from((event.1, PLAYER_Z_INDEX)),
            ))
            .id();
        player_mapping.0.insert(event.0, entity);
    }
}
