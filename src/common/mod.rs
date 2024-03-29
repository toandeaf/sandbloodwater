use crate::network::{DisconnectEvent, NewConnectionEvent};
use bevy::prelude::Event;
use serde::{Deserialize, Serialize};

use crate::player::{MovementEvent, PlayerCreateEvent, PlayerSyncEvent};

#[derive(Serialize, Deserialize, Event)]
pub enum EventWrapper {
    Test(String),
    Movement(MovementEvent),
    PlayerCreate(PlayerCreateEvent),
    PlayerSync(PlayerSyncEvent),
    NewConnection(NewConnectionEvent),
    Disconnect(DisconnectEvent),
}

pub const SERVER_ADDRESS: &str = "192.168.0.36:7878";

pub const EOF: u8 = 0x03;
