use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::ops::Range;

use crate::player::component::{AnimationTimer, Player};
use crate::player::entity::create_player_entity;
use crate::player::resource::PlayerAttributes;
use crate::world::TileType;

type Speed = f32;

// TODO this will probably need to get lifted out at some stage and associated with a component
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// TODO I'm hoping this isn't necessary once I crack this sticky/clippy issue with collisions.
// Note - it's currently decoupled from player speed, but they need to be in sync for smooth ops.
const COLLISION_BUFFER: f32 = 3.;
const DEFAULT_SPEED: f32 = 1.;
const PLAYER_Z_INDEX: f32 = 2.;

pub fn initialise_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_attributes: Res<PlayerAttributes>,
) {
    let window = window_query.get_single().unwrap();

    let half_window_width = window.width() / 2.;
    let half_window_height = window.height() / 2.;

    commands.spawn(create_player_entity(
        player_attributes,
        Vec3::new(half_window_width, half_window_height, PLAYER_Z_INDEX),
    ));
}

// TODO work out how to properly abstract those bundles to reduce complexity
#[allow(clippy::type_complexity)]
pub fn move_player(
    time: Res<Time>,
    player_attributes: Res<PlayerAttributes>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_bundle: Query<
        (&mut Transform, &mut AnimationTimer),
        (With<Player>, Without<TileType>),
    >,
    world_bundle: Query<(&Transform, &Sprite, &TileType), (With<TileType>, Without<Player>)>,
) {
    let player_radius = player_attributes.size / 2.;
    let player_speed = player_attributes.speed;

    for (mut player_transform, mut timer) in &mut player_bundle {
        let player_position = player_transform.translation;

        timer.tick(time.delta());

        if timer.just_finished() {
            let time_delta = time.delta_seconds();
            let adjusted_speed = player_speed * time_delta;

            keyboard_input
                .get_pressed()
                .for_each(|key_pressed| match key_pressed {
                    KeyCode::W => {
                        let speed_through_tile = calculate_speed_for_direction(
                            Direction::Up,
                            (player_position, player_radius),
                            &world_bundle,
                        );

                        player_transform.translation.y += adjusted_speed * speed_through_tile;
                    }
                    KeyCode::S => {
                        let speed_through_tile = calculate_speed_for_direction(
                            Direction::Down,
                            (player_position, player_radius),
                            &world_bundle,
                        );

                        player_transform.translation.y -= adjusted_speed * speed_through_tile;
                    }
                    KeyCode::A => {
                        let speed_through_tile = calculate_speed_for_direction(
                            Direction::Left,
                            (player_position, player_radius),
                            &world_bundle,
                        );

                        player_transform.translation.x -= adjusted_speed * speed_through_tile;
                    }
                    KeyCode::D => {
                        let speed_through_tile = calculate_speed_for_direction(
                            Direction::Right,
                            (player_position, player_radius),
                            &world_bundle,
                        );

                        player_transform.translation.x += adjusted_speed * speed_through_tile;
                    }
                    _ => {}
                });
        }
    }
}

#[allow(clippy::type_complexity)]
fn calculate_speed_for_direction(
    direction: Direction,
    player_attributes: (Vec3, f32),
    world_bundle: &Query<(&Transform, &Sprite, &TileType), (With<TileType>, Without<Player>)>,
) -> f32 {
    let (player_position, player_radius) = player_attributes;

    match direction {
        Direction::Up => calculate_speed_through_tile(
            world_bundle,
            (
                player_position.y + player_radius + COLLISION_BUFFER,
                player_position.x - player_radius,
                player_position.x + player_radius,
            ),
            compute_y_diameter_range,
            compute_x_diameter_range,
        ),
        Direction::Down => calculate_speed_through_tile(
            world_bundle,
            (
                player_position.y - player_radius - COLLISION_BUFFER,
                player_position.x - player_radius,
                player_position.x + player_radius,
            ),
            compute_y_diameter_range,
            compute_x_diameter_range,
        ),
        Direction::Left => calculate_speed_through_tile(
            world_bundle,
            (
                player_position.x - player_radius - COLLISION_BUFFER,
                player_position.y - player_radius,
                player_position.y + player_radius,
            ),
            compute_x_diameter_range,
            compute_y_diameter_range,
        ),
        Direction::Right => calculate_speed_through_tile(
            world_bundle,
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
    world_bundle: &Query<(&Transform, &Sprite, &TileType), (With<TileType>, Without<Player>)>,
    player_positions: (f32, f32, f32),
    compute_target_tile_range: fn(Vec3, f32) -> Range<f32>,
    compute_proximate_tile_range: fn(Vec3, f32) -> Range<f32>,
) -> Speed {
    // Contact point is a point on the player sprite's "border" that interfaces with the target tiles.
    // i.e.
    // If player is travelling up, contact point is the center of the top edge of the player sprite.
    // If player is travelling left, contact point is the center of the left edge of the player sprite.
    let (contact_point, player_left_side, player_right_side) = player_positions;

    for (tile_transform, sprite, tile_type) in world_bundle.iter() {
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

fn compute_x_diameter_range(position: Vec3, sprite_radius: f32) -> Range<f32> {
    (position.x - sprite_radius)..(position.x + sprite_radius)
}

fn compute_y_diameter_range(position: Vec3, sprite_radius: f32) -> Range<f32> {
    (position.y - sprite_radius)..(position.y + sprite_radius)
}
