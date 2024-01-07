use bevy::asset::Assets;
use bevy::prelude::{AssetServer, Commands, Handle, Image, Res, ResMut, Vec2, Vec3};
use bevy::sprite::TextureAtlas;

use crate::world::entity::{create_map_tile_entity, create_solid_map_tile_entity};
use crate::world::resource::{MapLayout, MapState};
use crate::world::TileType;

const TILE_SIZE: f32 = 30.0;

// Uses the asset server and the custom asset loader defined in world::resource to parse the tsv
// file and process the values into the Vec<Vec<TileType>> data structure.
pub fn load_map_asset(
    mut state: ResMut<MapState>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // The embedded assets plugin allows us to bundle our assets with our executable.
    // In this case, our wasm binary.
    state.land_handle = asset_server.load("embedded://land_layer.csv");
    state.solid_handle = asset_server.load("embedded://solid_layer.csv");
    state.water_handle = asset_server.load("embedded://water_layer.csv");

    let image_handle: Handle<Image> = asset_server.load("embedded://world/main.png");
    let texture_atlas =
        TextureAtlas::from_grid(image_handle, Vec2::new(32., 32.0), 46, 46, None, None);

    state.texture_handle = texture_atlases.add(texture_atlas);
}

// TODO CONSOLIDATE THIS MUTHA FUCKA
pub fn initialise_maps(
    mut commands: Commands,
    mut map_state: ResMut<MapState>,
    tsv_assets: ResMut<Assets<MapLayout>>,
    texture_atlas_assets: ResMut<Assets<TextureAtlas>>,
) {
    initialise_land(
        &mut commands,
        &map_state,
        &tsv_assets,
        &texture_atlas_assets,
    );
    initialise_water(
        &mut commands,
        &map_state,
        &tsv_assets,
        &texture_atlas_assets,
    );
    initialise_solid(
        &mut commands,
        &map_state,
        &tsv_assets,
        &texture_atlas_assets,
    );

    map_state.applied = true;
}

pub fn initialise_land(
    commands: &mut Commands,
    map_state: &ResMut<MapState>,
    tsv_assets: &ResMut<Assets<MapLayout>>,
    texture_atlas_assets: &ResMut<Assets<TextureAtlas>>,
) {
    let mut starting_x = TILE_SIZE / 2.;
    let mut starting_y = TILE_SIZE / 2.;

    let map_texture_loaded = texture_atlas_assets.contains(map_state.texture_handle.id());

    let land_handle = tsv_assets.get(map_state.land_handle.id());

    // If the map has already been applied or the map asset hasn't been loaded yet.
    if map_state.applied || !map_texture_loaded || land_handle.is_none() {
        return;
    }

    if let Some(map_content) = land_handle {
        let matrix = map_content.map_matrix.clone();

        for row in matrix {
            for tile_index in row {
                let tile_params = (
                    Vec3::new(starting_x, starting_y, 0.),
                    TILE_SIZE,
                    TileType::Land,
                    tile_index,
                    map_state.texture_handle.clone(),
                );

                commands.spawn(create_map_tile_entity(
                    tile_params.0,
                    tile_params.1,
                    tile_params.2,
                    tile_params.3,
                    tile_params.4,
                ));

                starting_x += TILE_SIZE;
            }
            starting_x = TILE_SIZE / 2.;
            starting_y += TILE_SIZE;
        }
    }
}

pub fn initialise_solid(
    commands: &mut Commands,
    map_state: &ResMut<MapState>,
    tsv_assets: &ResMut<Assets<MapLayout>>,
    texture_atlas_assets: &ResMut<Assets<TextureAtlas>>,
) {
    let mut starting_x = TILE_SIZE / 2.;
    let mut starting_y = TILE_SIZE / 2.;

    let map_texture_loaded = texture_atlas_assets.contains(map_state.texture_handle.id());

    let solid_handle = tsv_assets.get(map_state.solid_handle.id());

    // If the map has already been applied or the map asset hasn't been loaded yet.
    if map_state.applied || !map_texture_loaded || solid_handle.is_none() {
        return;
    }

    if let Some(map_content) = solid_handle {
        let matrix = map_content.map_matrix.clone();

        for row in matrix {
            for tile_type in row {
                let tile_params = (
                    Vec3::new(starting_x, starting_y, 3.),
                    TILE_SIZE,
                    TileType::Mountain,
                    tile_type,
                    map_state.texture_handle.clone(),
                );

                commands.spawn(create_solid_map_tile_entity(
                    tile_params.0,
                    tile_params.1,
                    tile_params.2,
                    tile_params.3,
                    tile_params.4,
                ));

                starting_x += TILE_SIZE;
            }
            starting_x = TILE_SIZE / 2.;
            starting_y += TILE_SIZE;
        }
    }
}

pub fn initialise_water(
    commands: &mut Commands,
    map_state: &ResMut<MapState>,
    tsv_assets: &ResMut<Assets<MapLayout>>,
    texture_atlas_assets: &ResMut<Assets<TextureAtlas>>,
) {
    let mut starting_x = TILE_SIZE / 2.;
    let mut starting_y = TILE_SIZE / 2.;

    let map_texture_loaded = texture_atlas_assets.contains(map_state.texture_handle.id());

    let water_handle = tsv_assets.get(map_state.water_handle.id());

    // If the map has already been applied or the map asset hasn't been loaded yet.
    if map_state.applied || !map_texture_loaded || water_handle.is_none() {
        return;
    }

    if let Some(map_content) = water_handle {
        let matrix = map_content.map_matrix.clone();

        for row in matrix {
            for tile_type in row {
                let tile_params = (
                    Vec3::new(starting_x, starting_y, 1.),
                    TILE_SIZE,
                    TileType::Water,
                    tile_type,
                    map_state.texture_handle.clone(),
                );

                commands.spawn(create_map_tile_entity(
                    tile_params.0,
                    tile_params.1,
                    tile_params.2,
                    tile_params.3,
                    tile_params.4,
                ));

                starting_x += TILE_SIZE;
            }
            starting_x = TILE_SIZE / 2.;
            starting_y += TILE_SIZE;
        }
    }
}
