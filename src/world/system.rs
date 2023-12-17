use crate::world::entity::create_map_tile_entity;
use crate::world::utils::load_map_config_from_file;
use bevy::prelude::Commands;

const TILE_SIZE: f32 = 30.0;

// TODO Add proper sizing to fit window
pub fn initialise_map(mut commands: Commands) {
    let mut starting_x = TILE_SIZE / 2.;
    let mut starting_y = TILE_SIZE / 2.;

    let tile_rows = load_map_config_from_file();

    for row in tile_rows {
        for tile in row {
            commands.spawn(create_map_tile_entity(
                TILE_SIZE,
                &starting_x,
                &starting_y,
                tile,
            ));
            starting_x += TILE_SIZE;
        }
        starting_x = TILE_SIZE / 2.;
        starting_y += TILE_SIZE;
    }
}
