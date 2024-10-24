use bevy::prelude::*;

pub struct RemoteClientPlugin;

impl Plugin for RemoteClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, remote_client_system);
    }
}

fn remote_client_system() {}