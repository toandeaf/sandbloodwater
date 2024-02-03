use bevy::prelude::Event;
use serde::{Deserialize, Serialize};

use crate::player::MovementEvent;

#[derive(Serialize, Deserialize, Event)]
pub enum EventWrapper {
    Test(String),
    Movement(MovementEvent),
}

pub const SERVER_ADDRESS: &str = "127.0.0.1:7878";
