use bevy::app::{App, Plugin, Startup};

pub struct CommonEventsProcessingPlugin;

impl Plugin for CommonEventsProcessingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, example);
    }
}

fn example() {}
