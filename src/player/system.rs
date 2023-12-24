use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::ops::Range;

use crate::player::component::{AnimationTimer, Player};
use crate::player::entity::create_player_entity;
use crate::player::resource::PlayerAttributes;
use crate::world::TileType;

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
        Vec3::new(half_window_width, half_window_height, 1.),
    ));
}

// TODO this shuts things up, but bevy doesn't see it as a system param anymore wub wub
// type PlayerQuery<'a, 'b> =
//     Query<'a, 'b, (&'a mut Transform, &'b mut AnimationTimer), (With<Player>, Without<TileType>)>;

// TODO clean up these types somehow?
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
    for (mut player_transform, mut timer) in &mut player_bundle {
        let player_position = player_transform.translation;

        timer.tick(time.delta());

        if timer.just_finished() {
            let time_delta = time.delta_seconds();

            keyboard_input
                .get_pressed()
                .for_each(|key_pressed| match key_pressed {
                    KeyCode::W => {
                        let player_positions = (
                            player_position.y + player_radius,
                            player_position.x - player_radius,
                            player_position.x + player_radius,
                        );

                        if !collide_check(
                            &world_bundle,
                            player_positions,
                            compute_tile_bottom,
                            compute_tile_range_x,
                        ) {
                            player_transform.translation.y += player_attributes.speed * time_delta;
                        }
                    }
                    KeyCode::S => {
                        let player_positions = (
                            player_position.y - player_radius,
                            player_position.x - player_radius,
                            player_position.x + player_radius,
                        );

                        if !collide_check(
                            &world_bundle,
                            player_positions,
                            compute_tile_top,
                            compute_tile_range_x,
                        ) {
                            player_transform.translation.y -= player_attributes.speed * time_delta;
                        }
                    }
                    KeyCode::A => {
                        let player_positions = (
                            player_position.x - player_radius,
                            player_position.y - player_radius,
                            player_position.y + player_radius,
                        );

                        if !collide_check(
                            &world_bundle,
                            player_positions,
                            compute_tile_right,
                            compute_tile_range_y,
                        ) {
                            player_transform.translation.x -= player_attributes.speed * time_delta;
                        }
                    }
                    KeyCode::D => {
                        let player_positions = (
                            player_position.x + player_radius,
                            player_position.y - player_radius,
                            player_position.y + player_radius,
                        );

                        if !collide_check(
                            &world_bundle,
                            player_positions,
                            compute_tile_left,
                            compute_tile_range_y,
                        ) {
                            player_transform.translation.x += player_attributes.speed * time_delta;
                        }
                    }
                    _ => {}
                });
        }
    }
}

fn collide_check(
    world_bundle: &Query<(&Transform, &Sprite, &TileType), (With<TileType>, Without<Player>)>,
    player_positions: (f32, f32, f32),
    compute_tile_distance: fn(Vec3, f32, f32) -> f32,
    compute_sile_side_range: fn(Vec3, f32) -> Range<f32>,
) -> bool {
    let (contact_edge, player_left_side, player_right_side) = player_positions;

    for (tile_transform, sprite, tile_type) in world_bundle.iter() {
        // TODO seems costly - abstract this to resource? Or figure out single queries?
        let sprite_radius = sprite.custom_size.map(|vec| vec.y).unwrap_or_default() / 2.;

        // The range from corner to corner of a given tile
        let tile_side_range = compute_sile_side_range(tile_transform.translation, sprite_radius);

        if tile_side_range.contains(&player_left_side)
            || tile_side_range.contains(&player_right_side)
        {
            // Distance between the y axis position of the player's top most edge and the tile's bottom most edge
            let distance =
                compute_tile_distance(tile_transform.translation, sprite_radius, contact_edge);

            // TODO This doesn't feel safe/consistent enough? Why's -3. fine?
            if (-3. ..0.).contains(&distance) && tile_is_solid(tile_type) {
                return true;
            }
        }
    }

    false
}

// TODO Suuurely I don't need this redundancy? Maybe just define the clojures inline?
fn compute_tile_top(tile_axis: Vec3, tile_radius: f32, contact_edge: f32) -> f32 {
    (tile_axis.y + tile_radius) - contact_edge
}

fn compute_tile_bottom(tile_axis: Vec3, tile_radius: f32, contact_edge: f32) -> f32 {
    contact_edge - (tile_axis.y - tile_radius)
}

fn compute_tile_right(tile_axis: Vec3, tile_radius: f32, contact_edge: f32) -> f32 {
    (tile_axis.x + tile_radius) - contact_edge
}

fn compute_tile_left(tile_axis: Vec3, tile_radius: f32, contact_edge: f32) -> f32 {
    contact_edge - (tile_axis.x - tile_radius)
}

fn compute_tile_range_x(position: Vec3, sprite_radius: f32) -> Range<f32> {
    (position.x - sprite_radius)..(position.x + sprite_radius)
}

fn compute_tile_range_y(position: Vec3, sprite_radius: f32) -> Range<f32> {
    (position.y - sprite_radius)..(position.y + sprite_radius)
}

fn tile_is_solid(tile_type: &TileType) -> bool {
    match tile_type {
        TileType::Land => false,
        TileType::Mountain => true,
        TileType::Water => false,
        TileType::Building => true,
        TileType::Unsupported => false,
    }
}
