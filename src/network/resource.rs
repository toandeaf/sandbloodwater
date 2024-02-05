use bevy::prelude::Event;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Event)]
pub struct NewConnectionEvent;
