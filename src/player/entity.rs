use bevy::prelude::*;
use bevy::sprite::SpriteBundle;
use bevy::utils::default;

use crate::player::component::{Activity, AnimationTimer, CurrentActivity, CurrentDirection, Direction, Player};
use crate::player::resource::PlayerAttributes;

const ANIMATION_SPEED: f32 = 0.025;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    state: PlayerState,
    timer: AnimationTimer,
    sprite_bundle: SpriteBundle,
}

#[derive(Bundle)]
pub struct PlayerState {
    direction: CurrentDirection,
    activity: CurrentActivity
}

pub fn create_player_entity(
    player_attributes: Res<PlayerAttributes>,
    starting_position: Vec3,
) -> PlayerBundle {
    PlayerBundle {
        player: Player,
        state: PlayerState {
            direction: CurrentDirection(Direction::Down),
            activity: CurrentActivity(Activity::Idle),
        },
        timer: AnimationTimer(Timer::from_seconds(ANIMATION_SPEED, TimerMode::Repeating)),
        sprite_bundle: SpriteBundle {
            transform: Transform {
                translation: starting_position,
                ..default()
            },
            sprite: Sprite {
                color: player_attributes.color,
                custom_size: Some(Vec2::splat(player_attributes.size)),
                ..default()
            },
            ..default()
        },
    }
}
