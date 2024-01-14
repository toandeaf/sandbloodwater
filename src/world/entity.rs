use bevy::prelude::*;
use bevy::utils::petgraph::matrix_graph::Zero;

use crate::item::Solid;
use crate::world::component::TileType;
use crate::world::entity::TileBundle::SolidTileBundle;

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

pub enum TileBundle {
    SolidTileBundle(SolidWorldTileBundle),
    NormalTileBundle(WorldTileBundle),
}

pub fn create_map_tile_entity(
    tile_position: Vec3,
    tile_size: f32,
    tile_type: TileType,
    sprite_index: usize,
    texture_atlas: Handle<TextureAtlas>,
) -> TileBundle {
    let world_tile_bundle = WorldTileBundle {
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
    };

    if tile_type.speed_modifier().is_zero() {
        return SolidTileBundle(SolidWorldTileBundle {
            world_tile_bundle,
            solid: Solid,
        });
    };

    TileBundle::NormalTileBundle(world_tile_bundle)
}
