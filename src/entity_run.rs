use bevy::prelude::*;

pub struct EntityRunPlugin;

impl Plugin for EntityRunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, entity_run_system);
    }
}

fn entity_run_system() {
    println!("Initialising entity run plugin.");
}