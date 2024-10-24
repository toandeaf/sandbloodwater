use bevy::prelude::*;

pub struct RemoteServerPlugin;

impl Plugin for RemoteServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, remote_server_system);
    }
}

fn remote_server_system() {}