use bevy::prelude::*;

use crate::world::component::{TileType, World};
use crate::world::utils::get_tile_color;

pub fn generate_map_tile_entity(
    tile_size: f32,
    starting_x: &f32,
    starting_y: &f32,
    tile: TileType,
) -> (World, SpriteBundle) {
    (
        World,
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(*starting_x, *starting_y, 0.),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::splat(tile_size)),
                color: get_tile_color(tile),
                ..default()
            },
            ..default()
        },
    )
}
