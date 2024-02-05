use crate::common::EventWrapper;
use crate::network::Client;
use bevy::prelude::*;
use bevy::utils::{HashMap, Uuid};
use serde::{Deserialize, Serialize};

use crate::player::entity::create_player_entity;

pub const PLAYER_Z_INDEX: f32 = 2.;

#[derive(Event, Serialize, Deserialize, Copy, Clone)]
pub struct PlayerCreateEvent(pub Uuid, pub Vec2);

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
    mut client: ResMut<Client>,
    mut player_mapping: ResMut<PlayerMapping>,
) {
    let texture_handle = asset_server.load("embedded://player/walk.png");

    let texture_atlas_walk =
        TextureAtlas::from_grid(texture_handle, Vec2::new(60.0, 60.0), 9, 4, None, None);

    let texture_atlas_handle = texture_atlases.add(texture_atlas_walk);

    commands.insert_resource(PlayerTextureAtlas(texture_atlas_handle.clone()));

    let beside_the_items_lol = Vec2::new(200., 100.);

    let player_uuid = Uuid::new_v4();

    let entity = commands
        .spawn(create_player_entity(
            player_uuid,
            texture_atlas_handle,
            Vec3::from((beside_the_items_lol, PLAYER_Z_INDEX)),
        ))
        .id();

    player_mapping.0.insert(player_uuid, entity);

    client.send_event(EventWrapper::PlayerCreate(PlayerCreateEvent(
        player_uuid,
        beside_the_items_lol,
    )));
}
