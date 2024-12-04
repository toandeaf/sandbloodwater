use crate::local::LocalRenderPlugin;
use crate::remote::RemoteClientPlugin;
use bevy::prelude::*;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LocalRenderPlugin)
            .add_plugins(RemoteClientPlugin)
            .add_systems(Startup, client_system);
    }
}

fn client_system() {}
