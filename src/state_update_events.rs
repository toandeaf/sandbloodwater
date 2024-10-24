use bevy::prelude::*;

pub struct StateUpdateEventsPlugin;

impl Plugin for StateUpdateEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, state_update_events_system);
    }
}

fn state_update_events_system() {
    println!("Initialising state events plugin.");
}

