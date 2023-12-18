use bevy::prelude::{Color, Resource};

const PLAYER_SIZE: f32 = 20.;
const PLAYER_SPEED: f32 = 300.;
const PLAYER_COLOR: Color = Color::rgb(120., 115., 107.);

#[derive(Resource)]
pub struct PlayerAttributes {
    pub size: f32,
    pub speed: f32,
    pub color: Color,
}

pub fn create_player_resources() -> PlayerAttributes {
    PlayerAttributes {
        size: PLAYER_SIZE,
        speed: PLAYER_SPEED,
        color: PLAYER_COLOR,
    }
}
