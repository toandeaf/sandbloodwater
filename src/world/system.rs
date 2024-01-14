use bevy::asset::Assets;
use bevy::prelude::{AssetServer, Commands, Handle, Image, Res, ResMut, Vec2};
use bevy::sprite::TextureAtlas;

use crate::world::resource::{MapHandles, MapLayout};

// Uses the asset server and the custom asset loader defined in world::resource to parse the tsv
// file and process the values into the Vec<Vec<TileType>> data structure.
pub fn init_map_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image_handle: Handle<Image> = asset_server.load("embedded://world/main.png");

    let texture_atlas =
        TextureAtlas::from_grid(image_handle, Vec2::new(32., 32.0), 46, 46, None, None);
    let texture_handle = texture_atlases.add(texture_atlas);

    // The embedded assets plugin allows us to bundle our assets with our executable.
    // In this case, our wasm binary.
    let land_handle: Handle<MapLayout> = asset_server.load("embedded://land_layer.csv");
    let solid_handle: Handle<MapLayout> = asset_server.load("embedded://solid_layer.csv");
    let water_handle: Handle<MapLayout> = asset_server.load("embedded://water_layer.csv");

    let map_state = MapHandles {
        water_handle,
        land_handle,
        solid_handle,
        texture_handle,
    };

    commands.insert_resource(map_state);
}
