use bevy::prelude::Resource;
use bevy::utils::Uuid;

const PLAYER_SIZE: f32 = 20.;
const PLAYER_SPEED: f32 = 300.;

#[derive(Resource)]
pub struct PlayerAttributes {
    pub size: f32,
    pub radius: f32,
    pub speed: f32,
}

#[derive(Resource)]
pub struct PlayerUuid(pub Uuid);

pub fn create_player_resources() -> PlayerAttributes {
    PlayerAttributes {
        size: PLAYER_SIZE,
        radius: PLAYER_SIZE / 2.,
        speed: PLAYER_SPEED,
    }
}
