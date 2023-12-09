use crate::player::component::{AnimationTimer, Name, Player};
use bevy::prelude::*;
use bevy::sprite::SpriteBundle;
use bevy::utils::default;

const PLAYER_SIZE: f32 = 40.;
const PLAYER_COLOR: Color = Color::ORANGE_RED;
const ANIMATION_SPEED: f32 = 0.025;

pub fn generate_player_entity(
    player_name: &str,
    starting_position: Vec3,
) -> (Player, Name, AnimationTimer, SpriteBundle) {
    (
        Player,
        Name(String::from(player_name)),
        AnimationTimer(Timer::from_seconds(ANIMATION_SPEED, TimerMode::Repeating)),
        SpriteBundle {
            transform: Transform {
                translation: starting_position,
                ..default()
            },
            sprite: Sprite {
                color: PLAYER_COLOR,
                custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                ..default()
            },
            ..default()
        },
    )
}
