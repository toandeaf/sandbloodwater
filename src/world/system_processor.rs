use bevy::asset::AssetEvent;
use bevy::prelude::*;

use crate::world::entity::{create_map_tile_entity, TileBundle};
use crate::world::resource::{MapHandles, MapLayout};
use crate::world::TileType;

const TILE_SIZE: f32 = 30.0;
const EMPTY_TILE_INDEX: usize = 460;

// Event reader is listening for asset creation events. This happens the moment we invoke and process
// `asset_serer.load`. When an asset is loaded, the handle is returned. We're essentially "caching" these
// handles when we allocate and insert them into the system as resources. Then we're able to inject resources
// into our various system functions, like the below.
// So what we're doing here is - when the assets are good to go, apply them!
pub fn process_map_asset_init(
    mut reader: EventReader<AssetEvent<MapLayout>>,
    mut commands: Commands,
    map_state: Res<MapHandles>,
    map_layouts: Res<Assets<MapLayout>>,
) {
    for event in reader.read() {
        let atlas_layout_handle = map_state.atlas_layout_handle.clone();
        let atlas_texture_handle = map_state.atlas_texture_handle.clone();

        let (map_content_opt, tile_type) =
            if event.is_loaded_with_dependencies(map_state.land_handle.id()) {
                (map_layouts.get(&map_state.land_handle), TileType::Land)
            } else if event.is_loaded_with_dependencies(map_state.water_handle.id()) {
                (map_layouts.get(&map_state.water_handle), TileType::Water)
            } else if event.is_loaded_with_dependencies(map_state.solid_handle.id()) {
                (map_layouts.get(&map_state.solid_handle), TileType::Mountain)
            } else {
                (None, TileType::Unsupported)
            };

        render_map_by_layer(
            &mut commands,
            atlas_layout_handle,
            atlas_texture_handle,
            map_content_opt,
            tile_type,
        );
    }
}

pub fn render_map_by_layer(
    commands: &mut Commands,
    atlas_layout_handle: Handle<TextureAtlasLayout>,
    atlas_texture_handle: Handle<Image>,
    map_content_opt: Option<&MapLayout>,
    tile_type: TileType,
) {
    let mut starting_x = TILE_SIZE / 2.;
    let mut starting_y = TILE_SIZE / 2.;

    if let Some(map_content) = map_content_opt {
        let matrix = map_content.map_matrix.clone();

        for row in matrix {
            for tile_index in row {
                // This will probably go when we refactor the map implementation. But the long and
                // short of it is -> if tile is "invisible" we don't spawn the component.
                // But we still need to increment on the placement.
                if !tile_index.eq(&EMPTY_TILE_INDEX) {
                    let tile_bundle = create_map_tile_entity(
                        Vec3::new(starting_x, starting_y, 0.),
                        tile_type,
                        tile_index,
                        atlas_layout_handle.clone(),
                        atlas_texture_handle.clone(),
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
