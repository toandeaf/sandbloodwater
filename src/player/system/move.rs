use std::ops::Range;

use bevy::prelude::*;

use crate::item::Solid;
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

                        let speed_through_tile =
                            calculate_collision_or_speed_adjustment_for_direction(
                                &tile_bundle,
                                &solids_bundle,
                                (player_position, player_radius),
                                Direction::Up,
                            );

                        player_transform.translation.y += adjusted_speed * speed_through_tile;
                    }
                    KeyCode::S => {
                        direction.0 = Direction::Down;

                        let speed_through_tile =
                            calculate_collision_or_speed_adjustment_for_direction(
                                &tile_bundle,
                                &solids_bundle,
                                (player_position, player_radius),
                                Direction::Down,
                            );

                        player_transform.translation.y -= adjusted_speed * speed_through_tile;
                    }
                    KeyCode::A => {
                        direction.0 = Direction::Left;

                        let speed_through_tile =
                            calculate_collision_or_speed_adjustment_for_direction(
                                &tile_bundle,
                                &solids_bundle,
                                (player_position, player_radius),
                                Direction::Left,
                            );

                        player_transform.translation.x -= adjusted_speed * speed_through_tile;
                    }
                    KeyCode::D => {
                        direction.0 = Direction::Right;

                        let speed_through_tile =
                            calculate_collision_or_speed_adjustment_for_direction(
                                &tile_bundle,
                                &solids_bundle,
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

#[allow(clippy::type_complexity)]
fn calculate_collision_or_speed_adjustment_for_direction(
    tile_bundle: &Query<(&Transform, &Sprite, &TileType), (With<TileType>, Without<Player>)>,
    solid_bundle: &Query<(&Transform, &Sprite), (With<Solid>, Without<Player>)>,
    player_attributes: (Vec3, f32),
    direction: Direction,
) -> Speed {
    let (player_position, player_radius) = player_attributes;

    match direction {
        Direction::Up => calculate_collision_or_speed_adjustment(
            tile_bundle,
            solid_bundle,
            (
                player_position.y + player_radius + COLLISION_BUFFER,
                player_position.x - player_radius,
                player_position.x + player_radius,
            ),
            compute_y_diameter_range,
            compute_x_diameter_range,
        ),
        Direction::Down => calculate_collision_or_speed_adjustment(
            tile_bundle,
            solid_bundle,
            (
                player_position.y - player_radius - COLLISION_BUFFER,
                player_position.x - player_radius,
                player_position.x + player_radius,
            ),
            compute_y_diameter_range,
            compute_x_diameter_range,
        ),
        Direction::Left => calculate_collision_or_speed_adjustment(
            tile_bundle,
            solid_bundle,
            (
                player_position.x - player_radius - COLLISION_BUFFER,
                player_position.y - player_radius,
                player_position.y + player_radius,
            ),
            compute_x_diameter_range,
            compute_y_diameter_range,
        ),
        Direction::Right => calculate_collision_or_speed_adjustment(
            tile_bundle,
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

// Collision detection and speed modification implemented in the same system.
// 1. Iterate through all entities that are "Solid" (contain the Solid marker component).
// 2. Filter out entities that aren't on the proximate axis.
// 3. Evaluate entities that are on the target axis.
// 4. If there is an overlap of player contact point with entity's relevant "side" -> return 0 speed modifier.
// 5. Iterate through all tile map entities.
// 6. Seem filter and evaluations as steps 2. and 3.
// 7. If there is an overlap of the player contact point with entity's (tile here)
//    relevant "side" -> return tile specific speed modifier.
#[allow(clippy::type_complexity)]
fn calculate_collision_or_speed_adjustment(
    tile_bundle: &Query<(&Transform, &Sprite, &TileType), (With<TileType>, Without<Player>)>,
    solid_bundle: &Query<(&Transform, &Sprite), (With<Solid>, Without<Player>)>,
    player_positions: (f32, f32, f32),
    compute_target_tile_range: fn(Vec3, f32) -> Range<f32>,
    compute_proximate_tile_range: fn(Vec3, f32) -> Range<f32>,
) -> Speed {
    for (target_transform, sprite) in solid_bundle.iter() {
        // Seeing if the player interacts with any "Solid" components first.
        let collision_eval = detect_player_component_interaction(
            player_positions,
            (target_transform, sprite),
            compute_target_tile_range,
            compute_proximate_tile_range,
            None,
        );

        if let Some(collision_speed_change) = collision_eval {
            // If there was a collision, return the speed adjustment (it'll return 0).
            return collision_speed_change;
        }
    }

    // If player hasn't collided with anything, we'll see what tile they're on and whether
    // that should affect the speed.
    for (tile_transform, sprite, tile_type) in tile_bundle.iter() {
        // Same component iteration logic, except we're going through the remaining tiles now
        let speed_change_eval = detect_player_component_interaction(
            player_positions,
            (tile_transform, sprite),
            compute_target_tile_range,
            compute_proximate_tile_range,
            Some(tile_type),
        );

        if let Some(speed_change_eval) = speed_change_eval {
            // If the player is "on" a tile, it'll return said tile's speed modifier.
            return speed_change_eval;
        }
    }

    DEFAULT_SPEED
}

fn detect_player_component_interaction(
    player_positions: (f32, f32, f32),
    target_data: (&Transform, &Sprite),
    compute_target_tile_range: fn(Vec3, f32) -> Range<f32>,
    compute_proximate_tile_range: fn(Vec3, f32) -> Range<f32>,
    tile_type: Option<&TileType>,
) -> Option<Speed> {
    // Contact point is a point on the player sprite's "border" that interfaces with the target tiles.
    // i.e.
    // If player is travelling up, contact point is the center of the top edge of the player sprite.
    // If player is travelling left, contact point is the center of the left edge of the player sprite.
    let (contact_point, player_left_side, player_right_side) = player_positions;

    let (transform, sprite) = target_data;

    let sprite_radius = sprite.custom_size.map(|vec| vec.y).unwrap_or_default() / 2.;

    // Used to evaluate the opposite axis the player is trying to traverse, i.e. if player is
    // going up (y axis), this will be evaluating each tile's x axis.
    let proximate_tile_range = compute_proximate_tile_range(transform.translation, sprite_radius);

    // If the left or right most edge of the player interacts with the tile, then it's worth
    // evaluating further.
    if proximate_tile_range.contains(&player_left_side)
        || proximate_tile_range.contains(&player_right_side)
    {
        // Used to evaluate the target axis in the same way as described above
        let target_tile_range = compute_target_tile_range(transform.translation, sprite_radius);

        if target_tile_range.contains(&contact_point) {
            return Some(
                tile_type
                    .map(|tile_type_local| tile_type_local.speed_modifier())
                    .unwrap_or(0.),
            );
        }
    }

    None
}

fn compute_x_diameter_range(position: Vec3, sprite_radius: f32) -> Range<f32> {
    (position.x - sprite_radius)..(position.x + sprite_radius)
}

fn compute_y_diameter_range(position: Vec3, sprite_radius: f32) -> Range<f32> {
    (position.y - sprite_radius)..(position.y + sprite_radius)
}
