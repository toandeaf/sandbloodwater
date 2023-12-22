use bevy::prelude::*;
use bevy::window::PrimaryWindow;

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
        timer.tick(time.delta());

        // TODO refactor check_if_colliding to be direction configurable
        if timer.just_finished() {
            if keyboard_input.pressed(KeyCode::W) {
                if !collide_check_up(player_transform.translation, player_radius, &world_bundle) {
                    player_transform.translation.y +=
                        player_attributes.speed * time.delta_seconds();
                }
            } else if keyboard_input.pressed(KeyCode::S) {
                if !collide_check_down(player_transform.translation, player_radius, &world_bundle) {
                    player_transform.translation.y -=
                        player_attributes.speed * time.delta_seconds();
                }
            } else if keyboard_input.pressed(KeyCode::A) {
                if !collide_check_left(player_transform.translation, player_radius, &world_bundle) {
                    player_transform.translation.x -=
                        player_attributes.speed * time.delta_seconds();
                }
            } else if keyboard_input.pressed(KeyCode::D) {
                if !collide_check_right(player_transform.translation, player_radius, &world_bundle)
                {
                    player_transform.translation.x +=
                        player_attributes.speed * time.delta_seconds();
                }
            }
        }
    }
}

fn collide_check_up(
    player_position: Vec3,
    player_radius: f32,
    world_bundle: &Query<(&Transform, &Sprite, &TileType), (With<TileType>, Without<Player>)>,
) -> bool {
    let contact_edge = player_position.y + player_radius;

    for (tile_transform, sprite, tile_type) in world_bundle.iter() {
        // TODO seems costly - abstract this to resource? Or figure out single queries?
        let sprite_radius = sprite.custom_size.map(|vec| vec.y).unwrap_or_default() / 2.;

        // From the left most point on the x, to the right most.
        let x_range = (tile_transform.translation.x - sprite_radius)
            ..(tile_transform.translation.x + sprite_radius);

        // So if either edge of the player is touching the x_range of the sprite, RIP.
        let player_left_side = player_position.x - player_radius;
        let player_right_side = player_position.x + player_radius;

        if !x_range.contains(&player_left_side) && !x_range.contains(&player_right_side) {
            continue;
        }

        // Distance between the y axis position of the player's top most edge and the tile's bottom most edge
        let distance = contact_edge - (tile_transform.translation.y - sprite_radius);

        // TODO This doesn't feel safe/consistent enough? Why's -3. fine?
        if (-3. ..=0.).contains(&distance) && tile_is_solid(tile_type) {
            return true;
        }
    }

    false
}

fn collide_check_down(
    player_position: Vec3,
    player_radius: f32,
    world_bundle: &Query<(&Transform, &Sprite, &TileType), (With<TileType>, Without<Player>)>,
) -> bool {
    let contact_edge = player_position.y - player_radius;

    for (tile_transform, sprite, tile_type) in world_bundle.iter() {
        let sprite_radius = sprite.custom_size.map(|vec| vec.y).unwrap_or_default() / 2.;

        let x_range = (tile_transform.translation.x - sprite_radius)
            ..(tile_transform.translation.x + sprite_radius);

        let player_left_side = player_position.x - player_radius;
        let player_right_side = player_position.x + player_radius;

        if !x_range.contains(&player_left_side) && !x_range.contains(&player_right_side) {
            continue;
        }

        let distance = contact_edge - (tile_transform.translation.y + sprite_radius);

        if (-3. ..=0.).contains(&distance) && tile_is_solid(tile_type) {
            return true;
        }
    }

    false
}

fn collide_check_left(
    player_position: Vec3,
    player_radius: f32,
    world_bundle: &Query<(&Transform, &Sprite, &TileType), (With<TileType>, Without<Player>)>,
) -> bool {
    let contact_edge = player_position.x - player_radius;

    for (tile_transform, sprite, tile_type) in world_bundle.iter() {
        let sprite_radius = sprite.custom_size.map(|vec| vec.y).unwrap_or_default() / 2.;

        let y_range = (tile_transform.translation.y - sprite_radius)
            ..(tile_transform.translation.y + sprite_radius);

        let player_top_side = player_position.y + player_radius;
        let player_bottom_side = player_position.y - player_radius;

        if !y_range.contains(&player_top_side) && !y_range.contains(&player_bottom_side) {
            continue;
        }

        let distance = contact_edge - (tile_transform.translation.x + sprite_radius);

        if (-3. ..=0.).contains(&distance) && tile_is_solid(tile_type) {
            return true;
        }
    }

    false
}

fn collide_check_right(
    player_position: Vec3,
    player_radius: f32,
    world_bundle: &Query<(&Transform, &Sprite, &TileType), (With<TileType>, Without<Player>)>,
) -> bool {
    let contact_edge = player_position.x + player_radius;

    for (tile_transform, sprite, tile_type) in world_bundle.iter() {
        let sprite_radius = sprite.custom_size.map(|vec| vec.y).unwrap_or_default() / 2.;

        let y_range = (tile_transform.translation.y - sprite_radius)
            ..(tile_transform.translation.y + sprite_radius);

        let player_top_side = player_position.y + player_radius;
        let player_bottom_side = player_position.y - player_radius;

        if !y_range.contains(&player_top_side) && !y_range.contains(&player_bottom_side) {
            continue;
        }

        let distance = contact_edge - (tile_transform.translation.x - sprite_radius);

        if (-3. ..=0.).contains(&distance) && tile_is_solid(tile_type) {
            return true;
        }
    }

    false
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
