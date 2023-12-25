use bevy::math::Vec3;
use bevy::prelude::Commands;

use crate::item::entity::create_item_entity;

pub fn initialise_item(mut commands: Commands) {
    commands.spawn(create_item_entity(Vec3::new(100., 100., 1.)));
    commands.spawn(create_item_entity(Vec3::new(200., 200., 1.)));
}
