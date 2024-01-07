use bevy::prelude::{Commands, Vec2};

use crate::item::entity::{
    create_large_item_entity, create_medium_item_entity, create_small_item_entity,
};

const SMALL_ITEM_SIZE_MAXIMUM: f32 = 10.;
const MEDIUM_ITEM_SIZE_MINIMUM: f32 = 10.1;
const LARGE_ITEM_SIZE_MINIMUM: f32 = 20.;

pub fn initialise_item(mut commands: Commands) {
    create_item(&mut commands, Vec2::new(100., 100.), None);
    create_item(&mut commands, Vec2::new(200., 100.), Some(19.));
    create_item(&mut commands, Vec2::new(200., 200.), Some(30.));
}

pub fn create_item(commands: &mut Commands, position: Vec2, size_opt: Option<f32>) {
    let size = size_opt.unwrap_or(SMALL_ITEM_SIZE_MAXIMUM);

    // TODO Replace this with player size?
    if size > LARGE_ITEM_SIZE_MINIMUM {
        commands.spawn(create_large_item_entity(position, size));
    } else if (MEDIUM_ITEM_SIZE_MINIMUM..LARGE_ITEM_SIZE_MINIMUM).contains(&size) {
        commands.spawn(create_medium_item_entity(position, size));
    } else {
        commands.spawn(create_small_item_entity(position, size));
    }
}
