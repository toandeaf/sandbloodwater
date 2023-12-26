use crate::player::entity::create_player_entity;
use crate::player::resource::PlayerAttributes;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const PLAYER_Z_INDEX: f32 = 2.;

pub fn initialise_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_attributes: Res<PlayerAttributes>,
) {
    let window = window_query.get_single().unwrap();

    let half_window_width = window.width() / 2.;
    let half_window_height = window.height() / 2.;

    commands.spawn(create_player_entity(
        player_attributes,
        Vec3::new(half_window_width, half_window_height, PLAYER_Z_INDEX),
    ));
}
