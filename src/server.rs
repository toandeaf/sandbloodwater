use crate::compute::ComputePlugin;
use crate::remote::RemoteServerPlugin;
use bevy::prelude::*;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ComputePlugin)
            .add_plugins(RemoteServerPlugin)
            .add_systems(Startup, server_system);
    }
}

fn server_system() {
    println!("Server starting.");
}
