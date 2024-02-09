use bevy::prelude::Event;
use bevy::utils::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Event)]
pub struct NewConnectionEvent(pub(crate) Uuid);

#[derive(Serialize, Deserialize, Event)]
pub struct DisconnectEvent(Uuid);
