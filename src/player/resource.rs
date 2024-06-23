use bevy::prelude::Resource;
use bevy::utils::Uuid;

#[derive(Resource)]
pub struct PlayerUuid(pub Uuid);