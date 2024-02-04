use crate::common::EventWrapper;
use bevy::prelude::{
    Commands, Entity, EventReader, Parent, Query, Res, ResMut, TextureAtlasSprite, Transform, With,
};

use crate::item::Item;
use crate::player::component::{CurrentDirection, Direction};
use crate::player::resource::PlayerAttributes;
use crate::player::system::r#move::MovementEvent;
use crate::player::system::PlayerMapping;

#[allow(clippy::type_complexity)]
pub fn process_position_change(
    mut event_reader: EventReader<MovementEvent>,
    mut movement_query: Query<(
        &mut Transform,
        &mut TextureAtlasSprite,
        &mut CurrentDirection,
    )>,
    player_mapping: Res<PlayerMapping>,
) {
    for event in event_reader.read() {
        let entity = &event.0;
        let direction = &event.1;
        let new_speed = &event.2;

        // TODO - this is absolutely broken when both players occupy the same index. Needs rework.
        let player_entity_equivalent_opt = player_mapping.0.get::<Entity>(entity);

        if let Some(player_entity_equivalent) = player_entity_equivalent_opt {
            let player_res = movement_query.get_mut(*player_entity_equivalent);

            if let Ok(player_bundle) = player_res {
                let (mut transform, mut movement_sprite_sheet, mut current_direction) =
                    player_bundle;

                // Update player direction
                current_direction.0 = *direction;
                // Update sprite tile
                movement_sprite_sheet.index =
                    calculate_next_sprite(direction, &movement_sprite_sheet.index);
                // Update position
                transform.translation = direction.new_position(transform.translation, *new_speed);
            }
        }
    }
}

pub fn process_direction_change(
    mut event_reader: EventReader<MovementEvent>,
    mut item_query: Query<&mut Transform, (With<Item>, With<Parent>)>,
    player_attributes: Res<PlayerAttributes>,
) {
    let player_radius = player_attributes.size / 2.;

    for event in event_reader.read() {
        for mut child_transform in item_query.iter_mut() {
            child_transform.translation = event.1.relative_child_direction_change(player_radius);
        }
    }
}

// I'll probably move away from coupling the texture atlas indices per direction with
// the actual direction enum itself. Probably build an abstraction around the texture atlas itself?
fn calculate_next_sprite(direction: &Direction, current_sprite_index: &usize) -> usize {
    // Check index range of current direction, if the current sprite index isn't within that
    // range then the direction has been changed, and the new index retured should be the first
    // index of our new direction.
    // Basically "Are we animating in the right direction? If no, start new direction animation.
    if !direction.sprite_indices().contains(current_sprite_index) {
        direction.sprite_indices().start
    } else {
        // In this block we *are* going in the same index, so we're basically going to the next frame
        // in the animation sheet by incrementing the current index.
        current_sprite_index + 1
    }
}
