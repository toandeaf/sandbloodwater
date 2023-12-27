use crate::item::component::{InteractionType, Interactive, Item, Solid};
use bevy::prelude::*;

const ITEM_Z_INDEX: f32 = 1.;

#[derive(Bundle)]
pub struct ItemBundle {
    item: Item,
    sprite_bundle: SpriteBundle,
}

#[derive(Bundle)]
pub struct SmallItemBundle {
    item_bundle: ItemBundle,
    interactive: Interactive,
}

#[derive(Bundle)]
pub struct MediumItemBundle {
    item_bundle: ItemBundle,
    interactive: Interactive,
}

#[derive(Bundle)]
pub struct LargeItemBundle {
    item_bundle: ItemBundle,
    solid: Solid,
}

fn create_item_entity(starting_position: Vec2, size: f32) -> ItemBundle {
    ItemBundle {
        item: Item,
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

pub fn create_small_item_entity(starting_position: Vec2, size: f32) -> SmallItemBundle {
    let item_bundle = create_item_entity(starting_position, size);
    SmallItemBundle {
        item_bundle,
        interactive: Interactive(InteractionType::Collect),
    }
}

pub fn create_medium_item_entity(starting_position: Vec2, size: f32) -> MediumItemBundle {
    let item_bundle = create_item_entity(starting_position, size);
    MediumItemBundle {
        item_bundle,
        interactive: Interactive(InteractionType::Carry),
    }
}

pub fn create_large_item_entity(starting_position: Vec2, size: f32) -> LargeItemBundle {
    let item_bundle = create_item_entity(starting_position, size);
    LargeItemBundle {
        item_bundle,
        solid: Solid,
    }
}
