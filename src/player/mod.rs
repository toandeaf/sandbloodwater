use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_action_system);
    }
}

fn player_action_system() {
    println!("Initialising player action plugin.");
}
