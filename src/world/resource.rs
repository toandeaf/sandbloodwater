use std::str::from_utf8;

use bevy::utils::thiserror;
use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    reflect::TypePath,
    utils::BoxedFuture,
};
use thiserror::Error;

use crate::world::utils::load_map_config_from_file;

// TODO - half considering introducing `asset.rs` as its own standard. Don't like muddying resources with it.
#[derive(Asset, TypePath, Clone)]
pub struct MapLayout {
    pub map_matrix: Vec<Vec<usize>>,
}

#[derive(Resource, Default)]
pub struct MapHandles {
    pub land_handle: Handle<MapLayout>,
    pub solid_handle: Handle<MapLayout>,
    pub water_handle: Handle<MapLayout>,
    pub texture_handle: Handle<TextureAtlas>,
}

#[derive(Default)]
pub struct MapLoader;

#[derive(Debug, Error)]
pub enum CustomAssetLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
}

// So we're defining our map asset loader. Has to implement AssetLoader and the below types and
// functions.
impl AssetLoader for MapLoader {
    // Should represent the asset type that it'll generate. Has to be an Asset.
    type Asset = MapLayout;
    // Don't need this, currently aren't using settings.
    type Settings = ();
    type Error = CustomAssetLoaderError;

    // The main function to implement for asset loader. We're only using the reader to the bytes
    // from the given file into. The function itself should return a boxed future of the Asset type
    // we defined above.
    // Was initially going to just return out the file's contents as a string and do the whole
    // TSV parsing upstream, but thought that given we'll eventually dynamically generate
    // maps, cleaner to encapsulate like this.
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();

            reader.read_to_end(&mut bytes).await?;

            let vals = from_utf8(&bytes).unwrap();

            let map_matrix = load_map_config_from_file(vals);

            Ok(MapLayout { map_matrix })
        })
    }

    // This is used to determine which loader gets allocated when we call `asset_server.load`.
    // If the file extension ends in tsv, we use this map loader. Pretty cool way implementation tbh.
    fn extensions(&self) -> &[&str] {
        &["csv"]
    }
}
