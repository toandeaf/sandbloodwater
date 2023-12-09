use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
