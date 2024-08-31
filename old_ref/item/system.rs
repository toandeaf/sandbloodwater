use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Handle, Image, Res, Vec2};

use crate::item::entity::{
    create_large_item_entity, create_medium_item_entity, create_small_item_entity,
};

const SMALL_ITEM_SIZE_MAXIMUM: f32 = 10.;
const MEDIUM_ITEM_SIZE_MINIMUM: f32 = 10.1;
const LARGE_ITEM_SIZE_MINIMUM: f32 = 20.;

pub fn initialise_item(mut commands: Commands, asset_server: Res<AssetServer>) {
    create_item(&mut commands, Vec2::new(100., 100.), None, &asset_server);
    create_item(
        &mut commands,
        Vec2::new(200., 100.),
        Some(19.),
        &asset_server,
    );
    create_item(
        &mut commands,
        Vec2::new(200., 200.),
        Some(30.),
        &asset_server,
    );
}

pub fn create_item(
    commands: &mut Commands,
    position: Vec2,
    size_opt: Option<f32>,
    asset_server: &Res<AssetServer>,
) {
    let size = size_opt.unwrap_or(SMALL_ITEM_SIZE_MAXIMUM);

    // TODO Replace this with player size?
    if size > LARGE_ITEM_SIZE_MINIMUM {
        let image_handle: Handle<Image> = asset_server.load("embedded://world/box.png");
        commands.spawn(create_large_item_entity(position, size, image_handle));
    } else if (MEDIUM_ITEM_SIZE_MINIMUM..LARGE_ITEM_SIZE_MINIMUM).contains(&size) {
        let image_handle: Handle<Image> = asset_server.load("embedded://world/box.png");

        commands.spawn(create_medium_item_entity(position, size, image_handle));
    } else {
        let image_handle: Handle<Image> = asset_server.load("embedded://world/box.png");
        commands.spawn(create_small_item_entity(position, size, image_handle));
    }
}
