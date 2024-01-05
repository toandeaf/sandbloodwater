use crate::player::entity::create_player_entity;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const PLAYER_Z_INDEX: f32 = 2.;

pub fn initialise_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let window = window_query.get_single().unwrap();

    let half_window_width = window.width() / 2.;
    let half_window_height = window.height() / 2.;

    let texture_handle = asset_server.load("embedded://player/walk.png");

    let texture_atlas_walk =
        TextureAtlas::from_grid(texture_handle, Vec2::new(60.0, 60.0), 9, 4, None, None);

    let texture_atlas_handle = texture_atlases.add(texture_atlas_walk);

    commands.spawn(create_player_entity(
        texture_atlas_handle,
        Vec3::new(half_window_width, half_window_height, PLAYER_Z_INDEX),
    ));
}
