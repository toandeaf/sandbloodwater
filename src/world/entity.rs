use crate::item::Solid;
use bevy::prelude::*;

use crate::world::component::{TileType, World};
use crate::world::utils::get_tile_color;

#[derive(Bundle)]
pub struct WorldTileBundle {
    world: World,
    sprite_bundle: SpriteBundle,
    tile_type: TileType,
}

#[derive(Bundle)]
pub struct SolidWorldTileBundle {
    world_tile_bundle: WorldTileBundle,
    solid: Solid,
}

const TILE_Z_INDEX: f32 = 0.;

pub fn create_solid_map_tile_entity(
    tile_position: Vec2,
    tile_size: f32,
    tile_type: TileType,
) -> SolidWorldTileBundle {
    let world_tile_bundle = create_map_tile_entity(tile_position, tile_size, tile_type);
    SolidWorldTileBundle {
        world_tile_bundle,
        solid: Solid,
    }
}

pub fn create_map_tile_entity(
    tile_position: Vec2,
    tile_size: f32,
    tile_type: TileType,
) -> WorldTileBundle {
    WorldTileBundle {
        world: World,
        sprite_bundle: SpriteBundle {
            transform: Transform {
                translation: Vec3::from((tile_position, TILE_Z_INDEX)),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::splat(tile_size)),
                color: get_tile_color(tile_type),
                ..default()
            },
            ..default()
        },
        tile_type,
    }
}
