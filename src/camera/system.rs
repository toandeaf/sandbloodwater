use bevy::prelude::{Vec3, Window, With};
use bevy::window::PrimaryWindow;

use crate::camera::entity::create_camera;

const TOP_LAYER_Z_INDEX: f32 = 1.;

pub fn initialise_camera(
    mut commands: bevy::prelude::Commands,
    window_query: bevy::prelude::Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    // Middle of window
    let x_pos = window.width() / 2.;
    let y_pos = window.height() / 2.;

    let camera_position = Vec3::new(x_pos, y_pos, TOP_LAYER_Z_INDEX);

    commands.spawn(create_camera(camera_position));
}
