use bevy::prelude::{Commands, Vec2};

use crate::item::entity::create_item_entity;

pub fn initialise_item(mut commands: Commands) {
    commands.spawn(create_item_entity(Vec2::new(100., 100.)));
    commands.spawn(create_item_entity(Vec2::new(200., 200.)));
}
