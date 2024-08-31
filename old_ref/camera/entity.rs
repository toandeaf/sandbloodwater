use bevy::prelude::*;

pub fn create_camera(Vec3 { x, y, z }: Vec3) -> Camera2dBundle {
    Camera2dBundle {
        transform: Transform::from_xyz(x, y, z),
        ..default()
    }
}
