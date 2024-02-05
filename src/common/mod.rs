use crate::network::NewConnectionEvent;
use bevy::prelude::Event;
use serde::{Deserialize, Serialize};

use crate::player::{MovementEvent, PlayerCreateEvent};

#[derive(Serialize, Deserialize, Event)]
pub enum EventWrapper {
    Test(String),
    Movement(MovementEvent),
    PlayerCreate(PlayerCreateEvent),
    NewConnectionEvent(NewConnectionEvent),
}

pub const SERVER_ADDRESS: &str = "127.0.0.1:7878";

pub const EOF: u8 = 0x03;
