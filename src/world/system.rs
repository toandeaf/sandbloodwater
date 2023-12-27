use crate::world::entity::{create_map_tile_entity, create_solid_map_tile_entity};
use crate::world::utils::load_map_config_from_file;
use bevy::prelude::{Commands, Vec2};

const TILE_SIZE: f32 = 30.0;

// TODO Add proper sizing to fit window
pub fn initialise_map(mut commands: Commands) {
    let mut starting_x = TILE_SIZE / 2.;
    let mut starting_y = TILE_SIZE / 2.;

    let tile_rows = load_map_config_from_file();

    for row in tile_rows {
        for tile_type in row {
            let tile_params = (Vec2::new(starting_x, starting_y), TILE_SIZE, tile_type);

            if tile_type.speed_modifier() == 0. {
                commands.spawn(create_solid_map_tile_entity(
                    tile_params.0,
                    tile_params.1,
                    tile_params.2,
                ));
            } else {
                commands.spawn(create_map_tile_entity(
                    tile_params.0,
                    tile_params.1,
                    tile_params.2,
                ));
            }

            starting_x += TILE_SIZE;
        }
        starting_x = TILE_SIZE / 2.;
        starting_y += TILE_SIZE;
    }
}
