use bevy::prelude::*;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// TODO Speed as a param for player
#[derive(Component)]
pub struct Player;

// TODO This will definitely be shared at some point - also can the enum just be the component?
#[derive(Component)]
pub struct CurrentDirection(pub Direction);

// TODO might just be able to use Res<Time> and predicate on seconds
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
