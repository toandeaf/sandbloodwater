use bevy::prelude::*;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, event_plugin);
    }
}

fn event_plugin() {
    println!("Initialising events plugin.");
}