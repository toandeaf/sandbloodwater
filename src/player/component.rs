use bevy::prelude::*;
use std::ops::Range;

#[derive(Component)]
pub struct Player;

// TODO might just be able to use Res<Time> and predicate on seconds
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct CurrentDirection(pub Direction);

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// TODO This will definitely be shared at some point - also can the enum just be the component?
#[derive(Component)]
pub struct CurrentActivity(pub Activity);

pub enum Activity {
    Idle,
    Carrying,
}

const COLLISION_BUFFER: f32 = 3.;

impl Direction {
    // TODO move this out to a texture handle abstraction containing these ranges
    pub fn sprite_indices(&self) -> Range<usize> {
        match *self {
            Direction::Up => 0..8,
            Direction::Down => 9..17,
            Direction::Left => 18..26,
            Direction::Right => 27..35,
        }
    }

    pub fn contact_point(&self, sprite_transform: Vec3, sprite_radius: f32) -> f32 {
        match *self {
            Direction::Up => sprite_transform.y + sprite_radius + COLLISION_BUFFER,
            Direction::Down => sprite_transform.y - sprite_radius - COLLISION_BUFFER,
            Direction::Left => sprite_transform.x - sprite_radius - COLLISION_BUFFER,
            Direction::Right => sprite_transform.x + sprite_radius + COLLISION_BUFFER,
        }
    }

    // TODO better naming for that
    pub fn opposite_axis_sides(&self, transform: Vec3, radius: f32) -> (f32, f32) {
        match *self {
            Direction::Up | Direction::Down => (transform.x - radius, transform.x + radius),
            Direction::Left | Direction::Right => (transform.y - radius, transform.y + radius),
        }
    }

    pub fn compute_target_range(&self, transform: Vec3, radius: f32) -> Range<f32> {
        match *self {
            Direction::Up | Direction::Down => (transform.y - radius)..(transform.y + radius),
            Direction::Left | Direction::Right => (transform.x - radius)..(transform.x + radius),
        }
    }

    pub fn compute_proxy_range(&self, transform: Vec3, radius: f32) -> Range<f32> {
        match *self {
            Direction::Up | Direction::Down => (transform.x - radius)..(transform.x + radius),
            Direction::Left | Direction::Right => (transform.y - radius)..(transform.y + radius),
        }
    }

    pub fn new_position(&self, mut sprite_transform: Vec3, speed_adjustment: f32) -> Vec3 {
        match *self {
            Direction::Up => sprite_transform.y += speed_adjustment,
            Direction::Down => sprite_transform.y -= speed_adjustment,
            Direction::Left => sprite_transform.x -= speed_adjustment,
            Direction::Right => sprite_transform.x += speed_adjustment,
        }

        sprite_transform
    }
}
