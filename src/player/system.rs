use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::player::component::{AnimationTimer, Player};
use crate::player::entity::create_player_entity;

const PLAYER_NAME: &str = "Ahman";
const PLAYER_SPEED: f32 = 300.;
const PLAYER_Z_INDEX: f32 = 1.;

pub fn initialise_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_window_width = window.width() / 2.;
    let half_window_height = window.height() / 2.;

    commands.spawn(create_player_entity(
        PLAYER_NAME,
        Vec3::new(half_window_width, half_window_height, PLAYER_Z_INDEX),
    ));
}

pub fn move_player(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_bundle: Query<(&mut Transform, &mut AnimationTimer), With<Player>>,
) {
    for (mut transform, mut timer) in &mut player_bundle {
        timer.tick(time.delta());
        if timer.just_finished() {
            if keyboard_input.pressed(KeyCode::W) {
                transform.translation.y += PLAYER_SPEED * time.delta_seconds();
            } else if keyboard_input.pressed(KeyCode::S) {
                transform.translation.y -= PLAYER_SPEED * time.delta_seconds();
            } else if keyboard_input.pressed(KeyCode::A) {
                transform.translation.x -= PLAYER_SPEED * time.delta_seconds();
            } else if keyboard_input.pressed(KeyCode::D) {
                transform.translation.x += PLAYER_SPEED * time.delta_seconds();
            }
        }
    }
}
