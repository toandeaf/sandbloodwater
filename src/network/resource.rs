use crate::common::EventWrapper;
use bevy::prelude::Event;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::net::TcpStream;
use std::sync::RwLock;

#[derive(Serialize, Deserialize, Event)]
pub struct NewConnectionEvent;

lazy_static! {
    pub static ref SESSION_CLIENTS: RwLock<HashMap<usize, TcpStream>> = RwLock::new(HashMap::new());
    pub static ref EVENT_QUEUE: RwLock<VecDeque<EventWrapper>> = RwLock::new(VecDeque::new());
}
