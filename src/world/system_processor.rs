use bevy::asset::AssetEvent;
use bevy::prelude::*;

use crate::world::entity::{create_map_tile_entity, TileBundle};
use crate::world::resource::{MapHandles, MapLayout};
use crate::world::TileType;

const TILE_SIZE: f32 = 30.0;
const EMPTY_TILE_INDEX: usize = 460;

// TODO biggest blocker - it looks like the "land" layer is overriding the collision detection.
// The tile collision works "fine" when the land layer isn't present.
pub fn process_map_asset_init(
    mut reader: EventReader<AssetEvent<MapLayout>>,
    mut commands: Commands,
    map_state: Res<MapHandles>,
    map_layouts: Res<Assets<MapLayout>>,
) {
    for event in reader.read() {
        let texture_handle = &map_state.texture_handle;

        let (map_content_opt, tile_type) = if event.is_added(map_state.land_handle.id()) {
            (map_layouts.get(&map_state.land_handle), TileType::Land)
        } else if event.is_added(map_state.water_handle.id()) {
            (map_layouts.get(&map_state.water_handle), TileType::Water)
        } else if event.is_added(map_state.solid_handle.id()) {
            (map_layouts.get(&map_state.solid_handle), TileType::Mountain)
        } else {
            (None, TileType::Unsupported)
        };

        render_map_by_layer(&mut commands, texture_handle, map_content_opt, tile_type);
    }
}

pub fn render_map_by_layer(
    commands: &mut Commands,
    texture_handle: &Handle<TextureAtlas>,
    map_content_opt: Option<&MapLayout>,
    tile_type: TileType,
) {
    let mut starting_x = TILE_SIZE / 2.;
    let mut starting_y = TILE_SIZE / 2.;

    if let Some(map_content) = map_content_opt {
        let matrix = map_content.map_matrix.clone();

        for row in matrix {
            for tile_index in row {
                if !tile_index.eq(&EMPTY_TILE_INDEX) {
                    let tile_bundle = create_map_tile_entity(
                        Vec3::new(starting_x, starting_y, 0.),
                        TILE_SIZE,
                        tile_type,
                        tile_index,
                        texture_handle.clone(),
                    );

                    match tile_bundle {
                        TileBundle::SolidTileBundle(solid_tile_bundle) => {
                            commands.spawn(solid_tile_bundle);
                        }
                        TileBundle::NormalTileBundle(normal_tile_bundle) => {
                            commands.spawn(normal_tile_bundle);
                        }
                    }
                }

                starting_x += TILE_SIZE;
            }
            starting_x = TILE_SIZE / 2.;
            starting_y += TILE_SIZE;
        }
    }
}
