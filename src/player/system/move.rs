use std::ops::Range;

use crate::item::Solid;
use bevy::prelude::*;

use crate::player::component::{AnimationTimer, CurrentDirection, Direction, Player};
use crate::player::resource::PlayerAttributes;
use crate::world::TileType;

type Speed = f32;

// TODO I'm hoping this isn't necessary once I crack this sticky/clippy issue with collisions.
// Note - it's currently decoupled from player speed, but they need to be in sync for smooth ops.
const COLLISION_BUFFER: f32 = 3.;
const DEFAULT_SPEED: f32 = 1.;

// TODO work out how to properly abstract those bundles to reduce complexity
#[allow(clippy::type_complexity)]
pub fn move_player(
    time: Res<Time>,
    player_attributes: Res<PlayerAttributes>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_bundle: Query<
        (&mut Transform, &mut AnimationTimer, &mut CurrentDirection),
        (With<Player>, Without<TileType>),
    >,
    tile_bundle: Query<(&Transform, &Sprite, &TileType), (With<TileType>, Without<Player>)>,
    solids_bundle: Query<(&Transform, &Sprite), (With<Solid>, Without<Player>)>,
) {
    let player_radius = player_attributes.size / 2.;
    let player_speed = player_attributes.speed;

    for (mut player_transform, mut timer, mut direction) in &mut player_bundle {
        let player_position = player_transform.translation;

        timer.tick(time.delta());

        if timer.just_finished() {
            let time_delta = time.delta_seconds();
            let adjusted_speed = player_speed * time_delta;

            keyboard_input
                .get_pressed()
                .for_each(|key_pressed| match key_pressed {
                    KeyCode::W => {
                        direction.0 = Direction::Up;

                        let collided = calculate_collision_for_direction(
                            &solids_bundle,
                            (player_position, player_radius),
                            Direction::Up,
                        );

                        if collided {
                            return;
                        }

                        let speed_through_tile = calculate_speed_for_direction(
                            &tile_bundle,
                            (player_position, player_radius),
                            Direction::Up,
                        );

                        player_transform.translation.y += adjusted_speed * speed_through_tile;
                    }
                    KeyCode::S => {
                        direction.0 = Direction::Down;

                        let collided = calculate_collision_for_direction(
                            &solids_bundle,
                            (player_position, player_radius),
                            Direction::Down,
                        );

                        if collided {
                            return;
                        }

                        let speed_through_tile = calculate_speed_for_direction(
                            &tile_bundle,
                            (player_position, player_radius),
                            Direction::Down,
                        );

                        player_transform.translation.y -= adjusted_speed * speed_through_tile;
                    }
                    KeyCode::A => {
                        direction.0 = Direction::Left;

                        let collided = calculate_collision_for_direction(
                            &solids_bundle,
                            (player_position, player_radius),
                            Direction::Left,
                        );

                        if collided {
                            return;
                        }

                        let speed_through_tile = calculate_speed_for_direction(
                            &tile_bundle,
                            (player_position, player_radius),
                            Direction::Left,
                        );

                        player_transform.translation.x -= adjusted_speed * speed_through_tile;
                    }
                    KeyCode::D => {
                        direction.0 = Direction::Right;

                        let collided = calculate_collision_for_direction(
                            &solids_bundle,
                            (player_position, player_radius),
                            Direction::Right,
                        );

                        if collided {
                            return;
                        }

                        let speed_through_tile = calculate_speed_for_direction(
                            &tile_bundle,
                            (player_position, player_radius),
                            Direction::Right,
                        );

                        player_transform.translation.x += adjusted_speed * speed_through_tile;
                    }
                    _ => {}
                });
        }
    }
}

fn calculate_collision_for_direction(
    solid_bundle: &Query<(&Transform, &Sprite), (With<Solid>, Without<Player>)>,
    player_attributes: (Vec3, f32),
    direction: Direction,
) -> bool {
    let (player_position, player_radius) = player_attributes;

    match direction {
        Direction::Up => calculate_collision_at_object(
            solid_bundle,
            (
                player_position.y + player_radius + COLLISION_BUFFER,
                player_position.x - player_radius,
                player_position.x + player_radius,
            ),
            compute_y_diameter_range,
            compute_x_diameter_range,
        ),
        Direction::Down => calculate_collision_at_object(
            solid_bundle,
            (
                player_position.y - player_radius - COLLISION_BUFFER,
                player_position.x - player_radius,
                player_position.x + player_radius,
            ),
            compute_y_diameter_range,
            compute_x_diameter_range,
        ),
        Direction::Left => calculate_collision_at_object(
            solid_bundle,
            (
                player_position.x - player_radius - COLLISION_BUFFER,
                player_position.y - player_radius,
                player_position.y + player_radius,
            ),
            compute_x_diameter_range,
            compute_y_diameter_range,
        ),
        Direction::Right => calculate_collision_at_object(
            solid_bundle,
            (
                player_position.x + player_radius + COLLISION_BUFFER,
                player_position.y - player_radius,
                player_position.y + player_radius,
            ),
            compute_x_diameter_range,
            compute_y_diameter_range,
        ),
    }
}

#[allow(clippy::type_complexity)]
fn calculate_speed_for_direction(
    tile_bundle: &Query<(&Transform, &Sprite, &TileType), (With<TileType>, Without<Player>)>,
    player_attributes: (Vec3, f32),
    direction: Direction,
) -> f32 {
    let (player_position, player_radius) = player_attributes;

    match direction {
        Direction::Up => calculate_speed_through_tile(
            tile_bundle,
            (
                player_position.y + player_radius + COLLISION_BUFFER,
                player_position.x - player_radius,
                player_position.x + player_radius,
            ),
            compute_y_diameter_range,
            compute_x_diameter_range,
        ),
        Direction::Down => calculate_speed_through_tile(
            tile_bundle,
            (
                player_position.y - player_radius - COLLISION_BUFFER,
                player_position.x - player_radius,
                player_position.x + player_radius,
            ),
            compute_y_diameter_range,
            compute_x_diameter_range,
        ),
        Direction::Left => calculate_speed_through_tile(
            tile_bundle,
            (
                player_position.x - player_radius - COLLISION_BUFFER,
                player_position.y - player_radius,
                player_position.y + player_radius,
            ),
            compute_x_diameter_range,
            compute_y_diameter_range,
        ),
        Direction::Right => calculate_speed_through_tile(
            tile_bundle,
            (
                player_position.x + player_radius + COLLISION_BUFFER,
                player_position.y - player_radius,
                player_position.y + player_radius,
            ),
            compute_x_diameter_range,
            compute_y_diameter_range,
        ),
    }
}

// Collision detection and speed adjustment are both implemented with this same system.
// 1. Iterate through each tile
// 2. Filter out all tiles that aren't on the player's non-target axis, i.e. if you're moving up,
//    we filter out all x tiles that don't touch the player's left OR right side.
// 3. Traverse the remaining tiles, finding their diameter range. If the player's contact point is
//    within that tile's diameter range, the player is "in" that tile.
// 4. Calculate tile's speed adjustment based on which tile the player is "in".
#[allow(clippy::type_complexity)]
fn calculate_speed_through_tile(
    tile_bundle: &Query<(&Transform, &Sprite, &TileType), (With<TileType>, Without<Player>)>,
    player_positions: (f32, f32, f32),
    compute_target_tile_range: fn(Vec3, f32) -> Range<f32>,
    compute_proximate_tile_range: fn(Vec3, f32) -> Range<f32>,
) -> Speed {
    // Contact point is a point on the player sprite's "border" that interfaces with the target tiles.
    // i.e.
    // If player is travelling up, contact point is the center of the top edge of the player sprite.
    // If player is travelling left, contact point is the center of the left edge of the player sprite.
    let (contact_point, player_left_side, player_right_side) = player_positions;

    for (tile_transform, sprite, tile_type) in tile_bundle.iter() {
        // TODO seems costly - abstract this to resource? Or figure out single queries?
        let sprite_radius = sprite.custom_size.map(|vec| vec.y).unwrap_or_default() / 2.;

        // Used to evaluate the opposite axis the player is trying to traverse, i.e. if player is
        // going up (y axis), this will be evaluating each tile's x axis.
        let proximate_tile_range =
            compute_proximate_tile_range(tile_transform.translation, sprite_radius);

        // If the left or right most edge of the player interacts with the tile, then it's worth
        // evaluating further.
        if proximate_tile_range.contains(&player_left_side)
            || proximate_tile_range.contains(&player_right_side)
        {
            // Used to evaluate the target axis in the same way as described above
            let target_tile_range =
                compute_target_tile_range(tile_transform.translation, sprite_radius);

            if target_tile_range.contains(&contact_point) {
                return tile_type.speed_modifier();
            }
        }
    }

    DEFAULT_SPEED
}

// Collision detection and speed adjustment are both implemented with this same system.
// 1. Iterate through each tile
// 2. Filter out all tiles that aren't on the player's non-target axis, i.e. if you're moving up,
//    we filter out all x tiles that don't touch the player's left OR right side.
// 3. Traverse the remaining tiles, finding their diameter range. If the player's contact point is
//    within that tile's diameter range, the player is "in" that tile.
// 4. Calculate tile's speed adjustment based on which tile the player is "in".
#[allow(clippy::type_complexity)]
fn calculate_collision_at_object(
    solid_bundle: &Query<(&Transform, &Sprite), (With<Solid>, Without<Player>)>,
    player_positions: (f32, f32, f32),
    compute_target_tile_range: fn(Vec3, f32) -> Range<f32>,
    compute_proximate_tile_range: fn(Vec3, f32) -> Range<f32>,
) -> bool {
    // Contact point is a point on the player sprite's "border" that interfaces with the target tiles.
    // i.e.
    // If player is travelling up, contact point is the center of the top edge of the player sprite.
    // If player is travelling left, contact point is the center of the left edge of the player sprite.
    let (contact_point, player_left_side, player_right_side) = player_positions;

    for (target_transform, sprite) in solid_bundle.iter() {
        // TODO seems costly - abstract this to resource? Or figure out single queries?
        let sprite_radius = sprite.custom_size.map(|vec| vec.y).unwrap_or_default() / 2.;

        // Used to evaluate the opposite axis the player is trying to traverse, i.e. if player is
        // going up (y axis), this will be evaluating each tile's x axis.
        let proximate_tile_range =
            compute_proximate_tile_range(target_transform.translation, sprite_radius);

        // If the left or right most edge of the player interacts with the tile, then it's worth
        // evaluating further.
        if proximate_tile_range.contains(&player_left_side)
            || proximate_tile_range.contains(&player_right_side)
        {
            // Used to evaluate the target axis in the same way as described above
            let target_tile_range =
                compute_target_tile_range(target_transform.translation, sprite_radius);

            if target_tile_range.contains(&contact_point) {
                return true;
            }
        }
    }

    false
}

fn compute_x_diameter_range(position: Vec3, sprite_radius: f32) -> Range<f32> {
    (position.x - sprite_radius)..(position.x + sprite_radius)
}

fn compute_y_diameter_range(position: Vec3, sprite_radius: f32) -> Range<f32> {
    (position.y - sprite_radius)..(position.y + sprite_radius)
}
