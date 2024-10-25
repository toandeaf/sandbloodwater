use bevy::prelude::*;

pub struct WorldTriggerPlugin;

impl Plugin for WorldTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, server_system);
    }
}

fn server_system() {
    println!("Initialising response run plugin.");
}
