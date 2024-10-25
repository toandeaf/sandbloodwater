use bevy::app::{App, Plugin, Startup};

pub struct MutateEventsProcessingPlugin;

impl Plugin for MutateEventsProcessingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, example);
    }
}

fn example() {}
