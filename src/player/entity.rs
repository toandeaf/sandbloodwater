use bevy::prelude::*;
use bevy::sprite::SpriteBundle;
use bevy::utils::default;

use crate::player::component::{AnimationTimer, Name, Player};

const PLAYER_SIZE: f32 = 20.;
const PLAYER_COLOR: Color = Color::rgb(120., 115., 107.);
const ANIMATION_SPEED: f32 = 0.025;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    name: Name,
    timer: AnimationTimer,
    sprite_bundle: SpriteBundle,
}

pub fn create_player_entity(player_name: &str, starting_position: Vec3) -> PlayerBundle {
    PlayerBundle {
        player: Player,
        name: Name(String::from(player_name)),
        timer: AnimationTimer(Timer::from_seconds(ANIMATION_SPEED, TimerMode::Repeating)),
        sprite_bundle: SpriteBundle {
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
    }
}
