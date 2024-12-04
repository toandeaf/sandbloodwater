use crate::compute::ComputePlugin;
use crate::local::LocalRenderPlugin;
use bevy::prelude::*;

pub struct StandalonePlugin;

impl Plugin for StandalonePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ComputePlugin)
            .add_plugins(LocalRenderPlugin)
            .add_systems(Startup, standalone_system);
    }
}

fn standalone_system() {}
