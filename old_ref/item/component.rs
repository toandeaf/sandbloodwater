use bevy::prelude::Component;

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct Solid;

#[derive(Component)]
pub struct Interactive(pub InteractionType);

pub enum InteractionType {
    Carry,
    Collect,
}
