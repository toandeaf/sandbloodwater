use bevy::prelude::*;
use entity_response::EntityResponsePlugin;
use world_response::WorldResponsePlugin;

mod entity_response;
mod world_response;

pub struct ResponseSystemsPlugin;

impl Plugin for ResponseSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EntityResponsePlugin, WorldResponsePlugin));
    }
}
