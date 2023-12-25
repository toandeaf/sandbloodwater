use crate::item::component::Item;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct ItemBundle {
    item: Item,
    sprite_bundle: SpriteBundle,
}

pub fn create_item_entity(starting_position: Vec3) -> ItemBundle {
    ItemBundle {
        item: Item,
        sprite_bundle: SpriteBundle {
            transform: Transform {
                translation: starting_position,
                ..default()
            },
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::splat(10.)),
                ..default()
            },
            ..default()
        },
    }
}
