use crate::player::component::{CurrentDirection, Player};
use crate::player::resource::PlayerAttributes;
use bevy::prelude::*;

#[derive(Event)]
pub struct DropEvent(pub Entity, Vec3);

pub fn drop(
    mut event_writer: EventWriter<DropEvent>,
    player_attributes: Res<PlayerAttributes>,
    keyboard_input: ResMut<Input<KeyCode>>,
    mut player_query: Query<(&Transform, Entity, &CurrentDirection), With<Player>>,
) {
    let player_radius = player_attributes.radius;

    for (player_transform, player_entity, current_direction) in &mut player_query {
        if keyboard_input.just_pressed(KeyCode::R) {
            let contact_position = current_direction
                .0
                .contact_position(player_transform.translation, player_radius);
            event_writer.send(DropEvent(player_entity, contact_position));
        }
    }
}
