use bevy::prelude::*;
use bevy::utils::Uuid;

use crate::player::component::{
    Activity, AnimationTimer, CharacterMarker, CurrentActivity, CurrentDirection, Direction, Player,
};
use crate::player::system::PlayerTextureAtlas;

// The higher this is, the slower the animation
const ANIMATION_SPEED: f32 = 0.040;
const PLAYER_SIZE: Option<Vec2> = Some(Vec2::new(40., 40.));

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
    player_atlas: PlayerTextureAtlas,
    starting_position: Vec3,
    direction: Direction,
) -> CharacterBundle {
    CharacterBundle {
        character_marker: CharacterMarker(uuid),
        state: PlayerState {
            direction: CurrentDirection(direction),
            activity: CurrentActivity(Activity::Idle),
        },
        timer: AnimationTimer(Timer::from_seconds(ANIMATION_SPEED, TimerMode::Repeating)),
        sprite_bundle: SpriteSheetBundle {
            transform: Transform {
                translation: starting_position,
                ..default()
            },
            sprite: Sprite {
                custom_size: PLAYER_SIZE,
                ..default()
            },
            atlas: TextureAtlas {
                layout: player_atlas.0,
                index: 0,
            },
            texture: player_atlas.1,
            ..default()
        },
    }
}

pub fn create_player_entity(
    uuid: Uuid,
    player_atlas: PlayerTextureAtlas,
    starting_position: Vec3,
    direction: Direction,
) -> PlayerBundle {
    PlayerBundle {
        player: Player,
        character_bundle: create_character_entity(uuid, player_atlas, starting_position, direction),
    }
}
