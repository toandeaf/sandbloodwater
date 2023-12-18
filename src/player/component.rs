use bevy::prelude::*;

// TODO Speed as a param for player
#[derive(Component)]
pub struct Player;

// TODO might just be able to use Res<Time> and predicate on seconds
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
