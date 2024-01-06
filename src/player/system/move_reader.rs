use bevy::prelude::{EventReader, Parent, Query, Res, TextureAtlasSprite, Transform, With};

use crate::item::Item;
use crate::player::component::{CurrentDirection, Direction, Player};
use crate::player::resource::PlayerAttributes;
use crate::player::system::r#move::MovementEvent;

// TODO refactor and rename these files. Probably have to set up a new standard for events?

#[allow(clippy::type_complexity)]
pub fn move_reader(
    mut event_reader: EventReader<MovementEvent>,
    mut player_query: Query<
        (
            &mut Transform,
            &mut TextureAtlasSprite,
            &mut CurrentDirection,
        ),
        With<Player>,
    >,
) {
    for event in event_reader.read() {
        let direction = &event.0;
        let new_speed = &event.1;

        let player_res = player_query.get_single_mut();

        if let Ok(player_bundle) = player_res {
            let (mut transform, mut movement_sprite_sheet, mut current_direction) = player_bundle;

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

pub fn process_movement(
    mut event_reader: EventReader<MovementEvent>,
    mut item_query: Query<&mut Transform, (With<Item>, With<Parent>)>,
    player_attributes: Res<PlayerAttributes>,
) {
    let player_radius = player_attributes.size / 2.;

    for event in event_reader.read() {
        for mut child_transform in item_query.iter_mut() {
            child_transform.translation = event.0.relative_child_direction_change(player_radius);
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
