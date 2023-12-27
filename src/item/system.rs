use bevy::prelude::{Commands, Vec2};

use crate::item::entity::{create_item_entity, create_large_item_entity};

pub fn initialise_item(mut commands: Commands) {
    create_item(&mut commands, Vec2::new(100., 100.), None);
    create_item(&mut commands, Vec2::new(200., 200.), Some(30.));
}

pub fn create_item(commands: &mut Commands, position: Vec2, size_opt: Option<f32>) {
    let size = size_opt.unwrap_or(10.);

    // TODO Replace this with player size?
    if size > 20. {
        commands.spawn(create_large_item_entity(position, size));
    } else {
        commands.spawn(create_item_entity(position, size));
    }
}
