use bevy::app::App;
use bevy::asset::AssetApp;
use bevy::prelude::{Plugin, Startup, Update};

use crate::world::resource::{MapLayout, MapLoader};
use crate::world::system::init_map_assets;
use crate::world::system_processor::process_map_asset_init;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<MapLayout>()
            .init_asset_loader::<MapLoader>()
            .add_systems(Startup, init_map_assets)
            .add_systems(Update, process_map_asset_init);
    }
}
