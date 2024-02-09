use crate::common::EventWrapper;
use bevy::prelude::Event;
use bevy::utils::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Event)]
pub struct NewConnectionEvent(pub(crate) String);

#[derive(Serialize, Deserialize, Event)]
pub struct DisconnectEvent(Uuid);

#[derive(Serialize, Deserialize, Event)]
pub struct NetworkWrapper(pub u8, pub EventWrapper);
