use crate::common::EventWrapper::PlayerCreate;
use bevy::prelude::*;
use bevy::utils::{HashMap, Uuid};
use serde::{Deserialize, Serialize};

use crate::player::component::Direction;
use crate::player::resource::PlayerUuid;

pub const PLAYER_Z_INDEX: f32 = 2.;

#[derive(Event, Serialize, Deserialize, Copy, Clone)]
pub struct PlayerCreateEvent(pub Uuid, pub Vec2, pub Direction);

#[derive(Event, Serialize, Deserialize, Copy, Clone)]
pub struct PlayerSyncEvent(pub Uuid, pub Vec2, pub Direction);

#[derive(Resource)]
pub struct PlayerMapping(pub HashMap<Uuid, Entity>);

#[derive(Resource)]
pub struct PlayerTextureAtlas(pub Handle<TextureAtlas>);

impl Default for PlayerTextureAtlas {
    fn default() -> Self {
        PlayerTextureAtlas(Handle::default())
    }
}

impl Default for PlayerMapping {
    fn default() -> Self {
        {
            PlayerMapping(HashMap::new())
        }
    }
}

pub fn initialise_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut event_writer: EventWriter<PlayerCreateEvent>,
    mut player_uuid: ResMut<PlayerUuid>,
) {
    let texture_handle = asset_server.load("embedded://player/walk.png");

    let texture_atlas_walk =
        TextureAtlas::from_grid(texture_handle, Vec2::new(60.0, 60.0), 9, 4, None, None);

    let texture_atlas_handle = texture_atlases.add(texture_atlas_walk);

    commands.insert_resource(PlayerTextureAtlas(texture_atlas_handle.clone()));

    let beside_the_items_lol = Vec2::new(200., 100.);

    player_uuid.0 = Uuid::new_v4();

    event_writer.send(PlayerCreateEvent(
        player_uuid.0,
        beside_the_items_lol,
        Direction::Down,
    ));
}
