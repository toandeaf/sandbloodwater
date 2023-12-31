use bevy::prelude::*;

use crate::item::Solid;
use crate::player::component::{AnimationTimer, Direction, Player};
use crate::player::resource::PlayerAttributes;
use crate::world::TileType;

pub type Speed = f32;
const DEFAULT_SPEED: f32 = 1.;
const DEFAULT_COLLISION_SPEED: Speed = 0.;

#[derive(Event)]
pub struct MovementEvent(pub Direction, pub Speed);

// TODO work out how to properly abstract those bundles to reduce complexity
#[allow(clippy::type_complexity)]
pub fn move_player(
    mut event_writer: EventWriter<MovementEvent>,
    mut player_query: Query<
        (&mut Transform, &mut AnimationTimer),
        (With<Player>, Without<TileType>),
    >,
    tile_query: Query<(&Transform, &Sprite, &TileType), (With<TileType>, Without<Player>)>,
    solid_query: Query<(&Transform, &Sprite), (With<Solid>, Without<Player>)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    player_attributes: Res<PlayerAttributes>,
) {
    let player_radius = player_attributes.radius;
    let player_base_speed = player_attributes.speed;

    for (player_transform, mut timer) in &mut player_query {
        timer.tick(time.delta());

        if timer.just_finished() {
            keyboard_input.get_pressed().for_each(|key_pressed| {
                let new_direction_opt = match key_pressed {
                    KeyCode::W => Some(Direction::Up),
                    KeyCode::S => Some(Direction::Down),
                    KeyCode::A => Some(Direction::Left),
                    KeyCode::D => Some(Direction::Right),
                    _ => None,
                };

                if let Some(direction) = new_direction_opt {
                    let player_data = (player_transform.translation, player_radius, &direction);

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

                    event_writer.send(MovementEvent(direction, new_speed));
                }
            });
        }
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

fn calculate_collision_or_speed_adjustment(
    tile_query: &Query<(&Transform, &Sprite, &TileType), (With<TileType>, Without<Player>)>,
    solid_query: &Query<(&Transform, &Sprite), (With<Solid>, Without<Player>)>,
    player_data: (Vec3, f32, &Direction),
) -> Speed {
    for (target_transform, sprite) in solid_query.iter() {
        // Seeing if the player interacts with any "Solid" components first.
        let collision_eval =
            detect_player_component_interaction(player_data, (target_transform, sprite, None));

        if let Some(collision_speed_change) = collision_eval {
            // If there was a collision, return the speed adjustment (it'll return 0).
            return collision_speed_change;
        }
    }

    // If player hasn't collided with anything, we'll see what tile they're on and whether
    // that should affect the speed.
    for (tile_transform, sprite, tile_type) in tile_query.iter() {
        // Same component iteration logic, except we're going through the remaining tiles now
        let speed_change_eval = detect_player_component_interaction(
            player_data,
            (tile_transform, sprite, Some(tile_type)),
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
    target_data: (&Transform, &Sprite, Option<&TileType>),
) -> Option<Speed> {
    // Deconstruct
    let (position, radius, direction) = player_data;
    let (transform, sprite, tile_type_opt) = target_data;

    // Contact point is a point on the player sprite's "border" that interfaces with the target tiles.
    // i.e. If player is travelling up, contact point is the center of the top edge of the player sprite.
    // i.e. If player is travelling left, contact point is the center of the left edge of the player sprite.
    let contact_point = direction.contact_point(position, radius);

    let (player_left_side, player_right_side) = direction.opposite_axis_sides(position, radius);

    let sprite_radius = sprite.custom_size.map(|vec| vec.y).unwrap_or_default() / 2.;

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
