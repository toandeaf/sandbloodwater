use crate::player::component::{Name, Player};

pub fn generate_player_entity(player_name: &str) -> (Player, Name) {
    (Player, Name(String::from(player_name)))
}
