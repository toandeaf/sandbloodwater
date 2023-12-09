use crate::player::component::{Name, Player};
use crate::player::entity::generate_player_entity;
use bevy::prelude::{Commands, Query, With};

pub fn add_players(mut commands: Commands) {
    commands.spawn(generate_player_entity("Jake"));
}

pub fn iterate_through_players(query: Query<&Name, With<Player>>) {
    for player in query.iter() {
        println!("Player {} has entered the game.", player.0);
    }
}
