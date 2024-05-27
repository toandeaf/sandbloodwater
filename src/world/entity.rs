use bevy::prelude::*;
use bevy::utils::petgraph::matrix_graph::Zero;

use crate::item::Solid;
use crate::world::component::TileType;
use crate::world::entity::TileBundle::{NormalTileBundle, SolidTileBundle};

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
    tile_type: TileType,
    sprite_index: usize,
    atlas_layout_handle: Handle<TextureAtlasLayout>,
    atlas_texture_handle: Handle<Image>,
) -> TileBundle {
    let world_tile_bundle = WorldTileBundle {
        sprite_bundle: SpriteSheetBundle {
            transform: Transform {
                translation: tile_position,
                ..default()
            },
            texture: atlas_texture_handle,
            atlas: TextureAtlas {
                layout: atlas_layout_handle,
                index: sprite_index,
            },
            sprite: Sprite::default(),
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

    NormalTileBundle(world_tile_bundle)
}
