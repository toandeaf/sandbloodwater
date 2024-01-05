use bevy::prelude::*;
use bevy::utils::default;

use crate::player::component::{
    Activity, AnimationTimer, CurrentActivity, CurrentDirection, Direction, Player,
};

// The higher this is, the slower the animation
const ANIMATION_SPEED: f32 = 0.040;
const PLAYER_SIZE: f32 = 40.;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    state: PlayerState,
    timer: AnimationTimer,
    sprite_bundle: SpriteSheetBundle,
}

#[derive(Bundle)]
pub struct PlayerState {
    direction: CurrentDirection,
    activity: CurrentActivity,
}

pub fn create_player_entity(
    texture_atlas: Handle<TextureAtlas>,
    starting_position: Vec3,
) -> PlayerBundle {
    PlayerBundle {
        player: Player,
        state: PlayerState {
            direction: CurrentDirection(Direction::Down),
            activity: CurrentActivity(Activity::Idle),
        },
        timer: AnimationTimer(Timer::from_seconds(ANIMATION_SPEED, TimerMode::Repeating)),
        sprite_bundle: SpriteSheetBundle {
            transform: Transform {
                translation: starting_position,
                ..default()
            },
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                ..default()
            },
            texture_atlas,
            ..default()
        },
    }
}
