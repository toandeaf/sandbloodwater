use bevy::prelude::*;

use crate::player::entity::create_player_entity;

const PLAYER_Z_INDEX: f32 = 2.;

pub fn initialise_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("embedded://player/walk.png");

    let texture_atlas_walk =
        TextureAtlas::from_grid(texture_handle, Vec2::new(60.0, 60.0), 9, 4, None, None);

    let texture_atlas_handle = texture_atlases.add(texture_atlas_walk);

    let beside_the_items_lol = Vec2::new(100., 200.);

    commands.spawn(create_player_entity(
        texture_atlas_handle,
        Vec3::from((beside_the_items_lol, PLAYER_Z_INDEX)),
    ));
}
