use bevy::app::App;
use bevy::asset::AssetApp;
use bevy::prelude::{Plugin, Startup, Update};

use crate::world::resource::{MapContent, MapLoader, MapState};
use crate::world::system::{initialise_map, load_map_asset};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<MapContent>()
            .init_asset_loader::<MapLoader>()
            .init_resource::<MapState>()
            .add_systems(Startup, load_map_asset)
            // I'm not a fan of this implementation. Even though I've the "is loaded" check
            // in the initialise map function itself, it just feels massively redundant.
            // TODO replace this with an event driven function that's only initialised on asset load
            .add_systems(Update, initialise_map);
    }
}
