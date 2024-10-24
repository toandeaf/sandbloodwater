use bevy::prelude::*;

pub struct WorldRunSystemsPlugin;

impl Plugin for WorldRunSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, server_system);
    }
}

fn server_system() {
    println!("Initialising world run plugin.");
}