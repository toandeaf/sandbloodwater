use bevy::asset::Assets;
use bevy::math::UVec2;
use bevy::prelude::{AssetServer, Commands, Handle, Image, Res, ResMut, TextureAtlasLayout};

use crate::world::resource::{MapHandles, MapLayout};

// First go at asset + resource loading.
// 1. Load the texture atlas image file into the asset server and are returned an image handle.
// 2. Generate a texture atlas handle using file specific texture atlas params and the image handle described above.
// 3. Add the texture atlas handle to the texture atlas assets resource.
// 4. Load each of the map layer csv files and hold onto their "MapLayout" handles.
// 5. Pull all the relevant handles into a single "MapHandles" struct.
// 6. Insert the "MapHandles" struct resource into the system. Making it available and firing off an "asset creation event".
pub fn init_map_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let atlas_texture_handle: Handle<Image> = asset_server.load("embedded://world/main.png");

    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 46, 46, None, None);
    let atlas_layout_handle = texture_atlases.add(texture_atlas);

    // The embedded assets plugin allows us to bundle our assets with our executable.
    // In this case, our wasm binary.
    let land_handle: Handle<MapLayout> = asset_server.load("embedded://land_layer.csv");
    let solid_handle: Handle<MapLayout> = asset_server.load("embedded://solid_layer.csv");
    let water_handle: Handle<MapLayout> = asset_server.load("embedded://water_layer.csv");

    let map_handles = MapHandles {
        water_handle,
        land_handle,
        solid_handle,
        atlas_layout_handle,
        atlas_texture_handle,
    };

    commands.insert_resource(map_handles);
}
