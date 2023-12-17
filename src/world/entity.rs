use bevy::prelude::*;

use crate::world::component::{TileType, World};

#[derive(Bundle)]
pub struct WorldTileBundle {
    world: World,
    sprite_bundle: SpriteBundle,
}

pub fn create_map_tile_entity(
    tile_size: f32,
    starting_x: &f32,
    starting_y: &f32,
    tile: TileType,
) -> WorldTileBundle {
    WorldTileBundle {
        world: World,
        sprite_bundle: SpriteBundle {
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
    }
}

const LAND_COLOR: (u8, u8, u8) = (201, 183, 123);
const MOUNTAIN_COLOR: (u8, u8, u8) = (145, 142, 132);
const WATER_COLOR: (u8, u8, u8) = (81, 129, 153);
const BUILDING_COLOR: (u8, u8, u8) = (156, 75, 40);
const UNSUPPORTED_COLOR: (u8, u8, u8) = (10, 10, 10);

fn get_tile_color(tile_type: TileType) -> Color {
    match tile_type {
        TileType::Land => Color::rgb_u8(LAND_COLOR.0, LAND_COLOR.1, LAND_COLOR.2),
        TileType::Mountain => Color::rgb_u8(MOUNTAIN_COLOR.0, MOUNTAIN_COLOR.1, MOUNTAIN_COLOR.2),
        TileType::Water => Color::rgb_u8(WATER_COLOR.0, WATER_COLOR.1, WATER_COLOR.2),
        TileType::Building => Color::rgb_u8(BUILDING_COLOR.0, BUILDING_COLOR.1, BUILDING_COLOR.2),
        TileType::Unsupported => Color::rgb_u8(
            UNSUPPORTED_COLOR.0,
            UNSUPPORTED_COLOR.1,
            UNSUPPORTED_COLOR.2,
        ),
    }
}
