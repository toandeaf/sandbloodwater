use bevy::prelude::*;

pub struct StateUpdateSystemsPlugin;

impl Plugin for StateUpdateSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, state_update_systems);
    }
}

fn state_update_systems() {
    println!("Initialising state events processing plugin.");
}