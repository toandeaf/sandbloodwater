use bevy::prelude::*;

use crate::item::Solid;
use crate::world::component::TileType;

#[derive(Bundle)]
pub struct WorldTileBundle {
    sprite_bundle: SpriteSheetBundle,
    tile_type: TileType,
}

#[derive(Bundle)]
pub struct SolidWorldTileBundle {
    world_tile_bundle: WorldTileBundle,
    solid: Solid,
}

pub fn create_solid_map_tile_entity(
    tile_position: Vec3,
    tile_size: f32,
    tile_type: TileType,
    sprite_index: usize,
    texture_atlas: Handle<TextureAtlas>,
) -> SolidWorldTileBundle {
    let world_tile_bundle = create_map_tile_entity(
        tile_position,
        tile_size,
        tile_type,
        sprite_index,
        texture_atlas,
    );
    SolidWorldTileBundle {
        world_tile_bundle,
        solid: Solid,
    }
}

pub fn create_map_tile_entity(
    tile_position: Vec3,
    tile_size: f32,
    tile_type: TileType,
    sprite_index: usize,
    texture_atlas: Handle<TextureAtlas>,
) -> WorldTileBundle {
    WorldTileBundle {
        sprite_bundle: SpriteSheetBundle {
            transform: Transform {
                translation: tile_position,
                ..default()
            },
            texture_atlas,
            sprite: TextureAtlasSprite {
                index: sprite_index,
                custom_size: Some(Vec2::splat(tile_size)),
                ..default()
            },
            ..default()
        },
        tile_type,
    }
}
