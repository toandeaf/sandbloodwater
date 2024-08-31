use bevy::prelude::*;

use crate::item::component::{InteractionType, Interactive, Item, Solid};

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

fn create_item_entity(
    starting_position: Vec2,
    size: f32,
    image_handle: Handle<Image>,
) -> ItemBundle {
    ItemBundle {
        item: Item,
        sprite_bundle: SpriteBundle {
            texture: image_handle,
            transform: Transform {
                translation: Vec3::from((starting_position, ITEM_Z_INDEX)),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            ..default()
        },
    }
}

pub fn create_small_item_entity(
    starting_position: Vec2,
    size: f32,
    image_handle: Handle<Image>,
) -> SmallItemBundle {
    let item_bundle = create_item_entity(starting_position, size, image_handle);
    SmallItemBundle {
        item_bundle,
        interactive: Interactive(InteractionType::Collect),
    }
}

pub fn create_medium_item_entity(
    starting_position: Vec2,
    size: f32,
    image_handle: Handle<Image>,
) -> MediumItemBundle {
    let item_bundle = create_item_entity(starting_position, size, image_handle);
    MediumItemBundle {
        item_bundle,
        interactive: Interactive(InteractionType::Carry),
    }
}

pub fn create_large_item_entity(
    starting_position: Vec2,
    size: f32,
    image_handle: Handle<Image>,
) -> LargeItemBundle {
    let item_bundle = create_item_entity(starting_position, size, image_handle);
    LargeItemBundle {
        item_bundle,
        solid: Solid,
    }
}
