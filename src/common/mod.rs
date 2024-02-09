use crate::network::{DisconnectEvent, NewConnectionEvent};
use bevy::prelude::Event;
use serde::{Deserialize, Serialize};

use crate::player::{MovementEvent, PlayerSyncEvent};

#[derive(Serialize, Deserialize, Event)]
pub enum EventWrapper {
    Test(String),
    Movement(MovementEvent),
    PlayerSync(PlayerSyncEvent),
    NewConnection(NewConnectionEvent),
    Disconnect(DisconnectEvent),
}

pub const SERVER_ADDRESS: &str = "127.0.0.1:7878";

pub const EOF: u8 = 0x03;
