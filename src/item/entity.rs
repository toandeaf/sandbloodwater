use crate::item::component::{Interactable, Item};
use bevy::prelude::*;

const ITEM_Z_INDEX: f32 = 1.;

#[derive(Bundle)]
pub struct ItemBundle {
    item: Item,
    interactable: Interactable,
    sprite_bundle: SpriteBundle,
}

pub fn create_item_entity(starting_position: Vec2) -> ItemBundle {
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
                custom_size: Some(Vec2::splat(10.)),
                ..default()
            },
            ..default()
        },
    }
}
