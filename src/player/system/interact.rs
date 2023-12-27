use bevy::prelude::*;

use crate::item::{create_item, Interactable};
use crate::player::component::{CurrentDirection, Direction, Player};
use crate::player::resource::PlayerAttributes;

const INTERACTION_PERIMETER: f32 = 10.;
const ITEM_SPAWN_SIZE: f32 = 10.;

// TODO this will eventually get split out into two separate systems.
// Eventually "interact" will yield options based on what the player can interact with.
// For the mean time it'll just pick items up in the perimeter.
#[allow(clippy::type_complexity)]
pub fn interact(
    mut commands: Commands,
    player_attributes: Res<PlayerAttributes>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut player_query: Query<(&Transform, &CurrentDirection), With<Player>>,
    item_query: Query<(&Transform, Entity), With<Interactable>>,
) {
    // TODO Player radius should go into player attributes as it won't fluctuate wildly.
    let player_radius = player_attributes.size / 2.;

    for (player_transform, current_direction) in &mut player_query {
        let player_position = player_transform.translation;

        if keyboard_input.pressed(KeyCode::E) {
            collect_item(player_position, player_radius, &mut commands, &item_query)
        } else if keyboard_input.pressed(KeyCode::R) {
            spawn_item(
                player_position,
                player_radius,
                &mut commands,
                &current_direction.0,
            );
            keyboard_input.reset(KeyCode::R);
        }
    }
}

// This function gets a perimeter around the player sprite. It then adds a perimeter within which
// items will be "interactable". If an item's center position is within this interaction perimeter,
// it will be "collected" (despawned).
fn collect_item(
    player_position: Vec3,
    player_radius: f32,
    commands: &mut Commands,
    item_query: &Query<(&Transform, Entity), With<Interactable>>,
) {
    let player_y_perimeter = player_position.y - (player_radius + INTERACTION_PERIMETER)
        ..player_position.y + (player_radius + INTERACTION_PERIMETER);

    let player_x_perimeter = player_position.x - (player_radius + INTERACTION_PERIMETER)
        ..player_position.x + (player_radius + INTERACTION_PERIMETER);

    for (transform, entity) in item_query.iter() {
        if player_x_perimeter.contains(&transform.translation.x)
            && player_y_perimeter.contains(&transform.translation.y)
        {
            commands
                .get_entity(entity)
                .iter_mut()
                .for_each(|entity| entity.despawn());
        }
    }
}

// This one's pretty self explanatory. It uses the "contact point" (the direction the player is facing's
// top most position on the relevant axis) and spawns a new item at that location.
fn spawn_item(
    player_position: Vec3,
    player_radius: f32,
    commands: &mut Commands,
    direction: &Direction,
) {
    let item_spawn_position: Vec2 = match direction {
        Direction::Up => Vec2::new(player_position.x, player_position.y + player_radius),
        Direction::Down => Vec2::new(player_position.x, player_position.y - player_radius),
        Direction::Left => Vec2::new(player_position.x - player_radius, player_position.y),
        Direction::Right => Vec2::new(player_position.x + player_radius, player_position.y),
    };

    create_item(commands, item_spawn_position, Some(ITEM_SPAWN_SIZE));
}
