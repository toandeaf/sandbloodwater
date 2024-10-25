use bevy::prelude::*;

pub struct EntityTriggerPlugin;

impl Plugin for EntityTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, entity_run_system);
    }
}

fn entity_run_system() {
    println!("Initialising entity run plugin.");
}
