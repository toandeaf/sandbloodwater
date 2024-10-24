use bevy::prelude::*;

pub struct WorldResponsePlugin;

impl Plugin for WorldResponsePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, world_response_systems);
    }
}

fn world_response_systems() {
    println!("Initialising world response plugin.");
}
