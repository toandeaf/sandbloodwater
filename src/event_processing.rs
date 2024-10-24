use bevy::prelude::*;

pub struct EventProcessingPlugin;

impl Plugin for EventProcessingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, event_processing);
    }
}

fn event_processing() {
    println!("Initialising events processing plugin.");
}