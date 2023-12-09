use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn initialise_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 1.),
        ..default()
    });
}
