use bevy::prelude::Event;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Event)]
pub enum EventId {
    Test(String),
    More(String),
    Movement(f32),
}
