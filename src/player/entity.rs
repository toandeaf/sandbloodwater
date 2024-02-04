use bevy::prelude::*;
use bevy::utils::{default, Uuid};

use crate::player::component::{
    Activity, AnimationTimer, CharacterMarker, CurrentActivity, CurrentDirection, Direction, Player,
};

// The higher this is, the slower the animation
const ANIMATION_SPEED: f32 = 0.040;
const PLAYER_SIZE: f32 = 40.;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    character_bundle: CharacterBundle,
}

#[derive(Bundle)]
pub struct CharacterBundle {
    character_marker: CharacterMarker,
    state: PlayerState,
    timer: AnimationTimer,
    sprite_bundle: SpriteSheetBundle,
}

#[derive(Bundle)]
pub struct PlayerState {
    direction: CurrentDirection,
    activity: CurrentActivity,
}

pub fn create_character_entity(
    uuid: Uuid,
    texture_atlas: Handle<TextureAtlas>,
    starting_position: Vec3,
) -> CharacterBundle {
    CharacterBundle {
        character_marker: CharacterMarker(uuid),
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

pub fn create_player_entity(
    uuid: Uuid,
    texture_atlas: Handle<TextureAtlas>,
    starting_position: Vec3,
) -> PlayerBundle {
    PlayerBundle {
        player: Player,
        character_bundle: create_character_entity(uuid, texture_atlas, starting_position),
    }
}
