use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct CurrentDirection(pub Direction);

// TODO This will definitely be shared at some point - also can the enum just be the component?
#[derive(Component)]
pub struct CurrentActivity(pub Activity);

// TODO might just be able to use Res<Time> and predicate on seconds
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum Activity {
    Idle,
    Carrying,
}
