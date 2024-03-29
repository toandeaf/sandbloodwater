use bevy::prelude::{App, Plugin, Startup};

use crate::camera::system::initialise_camera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialise_camera);
    }

    fn name(&self) -> &str {
        "camera_plugin"
    }

    fn is_unique(&self) -> bool {
        true
    }
}
