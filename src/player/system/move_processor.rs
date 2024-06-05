use bevy::math::Vec3;
use bevy::prelude::{EventReader, Parent, Query, Res, Sprite, TextureAtlas, Time, Transform, With, Without};
use bevy::utils::Uuid;

use crate::item::{Item, Solid};
use crate::player::component::{Attributes, CurrentDirection, Direction};
use crate::player::Player;
use crate::player::system::EntityMapping;
use crate::player::system::r#move::MovementEvent;
use crate::world::TileType;

const DEFAULT_SPEED: f32 = 1.;
const DEFAULT_COLLISION_SPEED: f32 = 0.;
const TILE_SIZE: f32 = 30.0;

#[allow(clippy::type_complexity)]
pub fn process_position_change(
    mut event_reader: EventReader<MovementEvent>,
    mut entity_movement_query: Query<(&mut Transform, &Attributes, &mut TextureAtlas, &mut CurrentDirection), (With<Player>, Without<TileType>)>,
    tile_query: Query<(&Transform, &Sprite, &TileType), (With<TileType>, Without<Player>)>,
    solid_query: Query<(&Transform, &Sprite), (With<Solid>, Without<Player>)>,
    entity_mapping: Res<EntityMapping>,
    time: Res<Time>,
) {
    for event in event_reader.read() {
        let uuid = &event.0;
        let direction = &event.1;

        // TODO - this is absolutely broken when both players occupy the same index. Needs rework.
        let ent_to_move_opt = entity_mapping.0.get::<Uuid>(uuid);

        if let Some(ent_to_move) = ent_to_move_opt {
            let player_res = entity_movement_query.get_mut(*ent_to_move);

            if let Ok(player_bundle) = player_res {
                let (mut transform, attributes, mut movement_sprite_sheet, mut current_direction) =
                    player_bundle;

                let player_base_speed = attributes.speed;
                let player_radius = attributes.radius;

                // Update player direction
                current_direction.0 = *direction;
                // Update sprite tile
                movement_sprite_sheet.index =
                    calculate_next_sprite(direction, &movement_sprite_sheet.index);

                let player_data = (transform.translation, player_radius, &current_direction.0);

                // TODO - collision detection borked after bevy bump.
                // AtlasSprite -> Sprite may have thrown the query off.
                let speed_modifier = calculate_collision_or_speed_adjustment(
                    &tile_query,
                    &solid_query,
                    player_data,
                );

                // Player's attribute speed multiplied by the speed adjustment from the tile contact.
                // The time.delta_seconds is to enforce the "real" speed. If we don't factor in
                // actual time into the computation, the clock speed of the processor will have an
                // effect on the actual speed of the game lol.
                let new_speed = player_base_speed * speed_modifier * time.delta_seconds();

                // Update position
                transform.translation = direction.new_position(transform.translation, new_speed);
            }
        }
    }
}

pub fn process_direction_change(
    mut event_reader: EventReader<MovementEvent>,
    mut item_query: Query<(&mut Transform, &Attributes), (With<Item>, With<Parent>)>,
) {
    for event in event_reader.read() {
        for (mut child_transform, attributes) in item_query.iter_mut() {
            child_transform.translation = event.1.relative_child_direction_change(attributes.radius);
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

// Collision detection and speed modification implemented in the same system.
// 1. Iterate through all entities that are "Solid" (contain the Solid marker component).
// 2. Filter out entities that aren't on the proximate axis.
// 3. Evaluate entities that are on the target axis.
// 4. If there is an overlap of player contact point with entity's relevant "side" -> return 0 speed modifier.
// 5. Iterate through all tile map entities.
// 6. Seem filter and evaluations as steps 2. and 3.
// 7. If there is an overlap of the player contact point with entity's (tile here) relevant "side" -> return tile specific speed modifier.
fn calculate_collision_or_speed_adjustment(
    tile_query: &Query<(&Transform, &Sprite, &TileType), (With<TileType>, Without<Player>)>,
    solid_query: &Query<(&Transform, &Sprite), (With<Solid>, Without<Player>)>,
    player_data: (Vec3, f32, &Direction),
) -> f32 {
    for (target_transform, sprite) in solid_query.iter() {
        // Seeing if the player interacts with any "Solid" components first.
        let sprite_radius = sprite.custom_size.map(|vec| vec.y).unwrap_or_default() / 2.;
        let collision_eval = detect_player_component_interaction(
            player_data,
            (target_transform, sprite_radius, None),
        );

        if let Some(collision_speed_change) = collision_eval {
            // If there was a collision, return the speed adjustment (it'll return 0).
            return collision_speed_change;
        }
    }

    // If player hasn't collided with anything, we'll see what tile they're on and whether
    // that should affect the speed.
    for (tile_transform, sprite, tile_type) in tile_query.iter() {
        // TODO rework map implementation. Just skipping "land" tiles for time being.
        // Because of how solids/mountain/water etc need to be "on" land (for it not to look shit)
        // when we're doing collision detection here we're iterating through the tiles the player
        // is "touching" and hitting Land tiles first. Lands speed modifier is 1, ergo 1 is speed mod.
        // It yeets out of the iter once we hit the first tile, despite the fact that the second tile is
        // water/mountain etc.
        // Was considering having land as z-index 0., all other "solid" tiles being 1. and then filtering
        // But feel like this should be best handled with a proper component implementation.
        // Possibly consolidate these two queries?? solid + tile?
        if let TileType::Land = tile_type {
            continue;
        }

        let sprite_radius = TILE_SIZE / 4.;

        // Same component iteration logic, except we're going through the remaining tiles now
        let speed_change_eval = detect_player_component_interaction(
            player_data,
            (tile_transform, sprite_radius, Some(tile_type)),
        );

        if let Some(speed_change_eval) = speed_change_eval {
            // If the player is "on" a tile, it'll return said tile's speed modifier.
            return speed_change_eval;
        }
    }

    DEFAULT_SPEED
}

fn detect_player_component_interaction(
    player_data: (Vec3, f32, &Direction),
    target_data: (&Transform, f32, Option<&TileType>),
) -> Option<f32> {
    // Deconstruct
    let (position, radius, direction) = player_data;
    let (transform, sprite_radius, tile_type_opt) = target_data;

    // Contact point is a point on the player sprite's "border" that interfaces with the target tiles.
    // i.e. If player is travelling up, contact point is the center of the top edge of the player sprite.
    // i.e. If player is travelling left, contact point is the center of the left edge of the player sprite.
    let contact_point = direction.contact_point(position, radius);

    let (player_left_side, player_right_side) = direction.opposite_axis_sides(position, radius);

    // Used to evaluate the opposite axis the player is trying to traverse,
    // i.e. if player is going up (y axis), this will be evaluating each tile's x axis.
    let proximate_tile_range = direction.compute_proxy_range(transform.translation, sprite_radius);

    // If the left or right most edge of the player interacts with the tile, then it's worth
    // evaluating further.
    if proximate_tile_range.contains(&player_left_side)
        || proximate_tile_range.contains(&player_right_side)
    {
        // Used to evaluate the target axis in the same way as described above
        let target_tile_range =
            direction.compute_target_range(transform.translation, sprite_radius);

        if target_tile_range.contains(&contact_point) {
            return tile_type_opt
                .map(|tile_type| tile_type.speed_modifier())
                .or(Some(DEFAULT_COLLISION_SPEED));
        }
    }

    None
}
