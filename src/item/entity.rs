use crate::item::component::{Interactable, Item, Solid};
use bevy::prelude::*;

const ITEM_Z_INDEX: f32 = 1.;

#[derive(Bundle)]
pub struct ItemBundle {
    item: Item,
    interactable: Interactable,
    sprite_bundle: SpriteBundle,
}

#[derive(Bundle)]
pub struct LargeItemBundle {
    item_bundle: ItemBundle,
    solid: Solid,
}

pub fn create_item_entity(starting_position: Vec2, size: f32) -> ItemBundle {
    ItemBundle {
        item: Item,
        interactable: Interactable,
        sprite_bundle: SpriteBundle {
            transform: Transform {
                translation: Vec3::from((starting_position, ITEM_Z_INDEX)),
                ..default()
            },
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            ..default()
        },
    }
}

// TODO medium items that can be carried
// pub fn create_medium_item(){
//
// }

pub fn create_large_item_entity(starting_position: Vec2, size: f32) -> LargeItemBundle {
    let item_bundle = create_item_entity(starting_position, size);
    LargeItemBundle {
        item_bundle,
        solid: Solid,
    }
}
