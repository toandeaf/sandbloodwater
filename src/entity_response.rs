use bevy::prelude::*;

pub struct EntityResponsePlugin;

impl Plugin for EntityResponsePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, entity_response_system);
    }
}

fn entity_response_system() {
    println!("Initialising entity response plugin.");
}