use bevy::prelude::*;

use crate::player::component::{Attributes, CurrentDirection, Player};

#[derive(Event)]
pub struct DropEvent(pub Entity, Vec3);

pub fn drop(
    mut event_writer: EventWriter<DropEvent>,
    keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Transform, Entity, &Attributes, &CurrentDirection), With<Player>>,
) {
    for (player_transform, player_entity, attributes, current_direction) in &mut player_query {
        if keyboard_input.just_pressed(KeyCode::KeyR) {
            let contact_position = current_direction
                .0
                .contact_position(player_transform.translation, attributes.radius);
            event_writer.send(DropEvent(player_entity, contact_position));
        }
    }
}
