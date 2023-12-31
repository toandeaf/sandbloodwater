use bevy::asset::Assets;
use bevy::prelude::{AssetServer, Commands, Res, ResMut, Vec2};

use crate::world::entity::{create_map_tile_entity, create_solid_map_tile_entity};
use crate::world::resource::{MapContent, MapState};

const TILE_SIZE: f32 = 30.0;

// Uses the asset server and the custom asset loader defined in world::resource to parse the tsv
// file and process the values into the Vec<Vec<TileType>> data structure.
pub fn load_map_asset(mut state: ResMut<MapState>, asset_server: Res<AssetServer>) {
    // The embedded assets plugin allows us to bundle our assets with our executable.
    // In this case, our wasm binary.
    state.handle = asset_server.load("embedded://map.tsv");
}

pub fn initialise_map(
    mut commands: Commands,
    mut map_state: ResMut<MapState>,
    tsv_assets: ResMut<Assets<MapContent>>,
) {
    let mut starting_x = TILE_SIZE / 2.;
    let mut starting_y = TILE_SIZE / 2.;

    let map_content_opt = tsv_assets.get(map_state.handle.id());

    // If the map has already been applied or the map asset hasn't been loaded yet.
    if map_state.applied || map_content_opt.is_none() {
        return;
    }

    if let Some(map_content) = map_content_opt {
        for row in &map_content.map_matrix {
            for tile_type in row {
                let tile_params = (Vec2::new(starting_x, starting_y), TILE_SIZE, tile_type);

                // Feels a bit hacky, but for now if the tile's speed modifier is 0 we're making the
                // assumption that the tile is and should be "solid".
                if tile_type.speed_modifier() == 0. {
                    commands.spawn(create_solid_map_tile_entity(
                        tile_params.0,
                        tile_params.1,
                        *tile_params.2,
                    ));
                } else {
                    commands.spawn(create_map_tile_entity(
                        tile_params.0,
                        tile_params.1,
                        *tile_params.2,
                    ));
                }

                starting_x += TILE_SIZE;
            }
            starting_x = TILE_SIZE / 2.;
            starting_y += TILE_SIZE;
        }

        // Once the actual tilemap entities have been spawned, we can update the map state
        // to be "applied".
        map_state.applied = true;
    }
}
