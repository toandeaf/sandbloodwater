use bevy::prelude::*;

use crate::item::{create_item, InteractionType, Interactive, Item};
use crate::player::component::{Activity, CurrentActivity, CurrentDirection, Direction, Player};
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
    mut player_query: Query<
        (&Transform, Entity, &CurrentDirection, &mut CurrentActivity),
        With<Player>,
    >,
    item_query: Query<(&Transform, Entity, &Interactive), With<Item>>,
    carrying_query: Query<&Children, With<Player>>,
) {
    // TODO Player radius should go into player attributes as it won't fluctuate wildly.
    let player_radius = player_attributes.size / 2.;

    for (player_transform, player_entity, current_direction, mut current_activity) in
        &mut player_query
    {
        let player_data = (
            player_transform.translation,
            player_radius,
            player_entity,
            &current_activity.0,
        );

        if keyboard_input.just_pressed(KeyCode::E) {
            // TODO split the activity up in here? Possibly return the available activity first?
            let new_activity =
                interact_with_item(&mut commands, &carrying_query, &item_query, player_data);

            if let Some(activity) = new_activity {
                current_activity.0 = activity;
            }
        } else if keyboard_input.just_pressed(KeyCode::R) {
            spawn_item(
                &mut commands,
                player_data.0,
                player_data.1,
                &current_direction.0,
            );
            keyboard_input.reset(KeyCode::R);
        }
    }
}

// This function gets a perimeter around the player sprite. It then adds a perimeter within which
// items will be "interactable". If an item's center position is within this interaction perimeter,
// it will be "collected" (despawned).
// TODO Comment this fucker more as it's not the most intuitive.
fn interact_with_item(
    commands: &mut Commands,
    carrying_query: &Query<&Children, With<Player>>,
    item_query: &Query<(&Transform, Entity, &Interactive), With<Item>>,
    player_data: (Vec3, f32, Entity, &Activity),
) -> Option<Activity> {
    let (player_position, player_radius, player_entity, player_activity) = player_data;

    // If the player is already carrying something, pressing this button will make them "drop" it.
    if let Activity::Carrying = player_activity {
        // Iterate through the player's children entities (at this stage it's just gonna be these items.
        // TODO this likely won't scale as we have more shit on the player?
        carrying_query.get_single().iter().for_each(|player_items| {
            for player_item in player_items.iter() {
                // Basically detach them from the parent in place.
                commands.entity(*player_item).remove_parent_in_place();
            }
        });

        return Some(Activity::Idle);
    };

    let player_y_perimeter = player_position.y - (player_radius + INTERACTION_PERIMETER)
        ..player_position.y + (player_radius + INTERACTION_PERIMETER);

    let player_x_perimeter = player_position.x - (player_radius + INTERACTION_PERIMETER)
        ..player_position.x + (player_radius + INTERACTION_PERIMETER);

    for (transform, entity, interactive) in item_query.iter() {
        if player_x_perimeter.contains(&transform.translation.x)
            && player_y_perimeter.contains(&transform.translation.y)
        {
            match interactive.0 {
                InteractionType::Collect => {
                    commands
                        .get_entity(entity)
                        .iter_mut()
                        .for_each(|entity| entity.despawn());
                }
                InteractionType::Carry => {
                    let mut item_entity = commands.entity(entity);

                    item_entity.set_parent_in_place(player_entity);

                    return Some(Activity::Carrying);
                }
            }
        }
    }

    None
}

// This one's pretty self explanatory. It uses the "contact point" (the direction the player is facing's
// top most position on the relevant axis) and spawns a new item at that location.
fn spawn_item(
    commands: &mut Commands,
    player_position: Vec3,
    player_radius: f32,
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
