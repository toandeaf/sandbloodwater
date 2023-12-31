use bevy::prelude::Color;

use crate::world::component::TileType;

const NEW_LINE_DELIMITER: char = '\n';
const TSV_DELIMITER: char = '\t';

pub fn load_map_config_from_file(map_string: &str) -> Vec<Vec<TileType>> {
    let lines = map_string.split(NEW_LINE_DELIMITER);

    let mut tile_rows: Vec<Vec<TileType>> = vec![];

    // We read these in from top left of the file, the opposite way we'd be rendering them.
    for line_res in lines {
        let mut characters: Vec<TileType> = vec![];
        for character in line_res.split(TSV_DELIMITER) {
            characters.push(character.parse::<TileType>().unwrap());
        }
        tile_rows.push(characters);
    }

    // So reverse the order so we read in from x = 0 and y = 0;
    tile_rows.reverse();

    tile_rows
}

const LAND_COLOR: (u8, u8, u8) = (201, 183, 123);
const MOUNTAIN_COLOR: (u8, u8, u8) = (145, 142, 132);
const WATER_COLOR: (u8, u8, u8) = (81, 129, 153);
const BUILDING_COLOR: (u8, u8, u8) = (156, 75, 40);
const UNSUPPORTED_COLOR: (u8, u8, u8) = (10, 10, 10);

pub fn get_tile_color(tile_type: TileType) -> Color {
    match tile_type {
        TileType::Land => Color::rgb_u8(LAND_COLOR.0, LAND_COLOR.1, LAND_COLOR.2),
        TileType::Mountain => Color::rgb_u8(MOUNTAIN_COLOR.0, MOUNTAIN_COLOR.1, MOUNTAIN_COLOR.2),
        TileType::Water => Color::rgb_u8(WATER_COLOR.0, WATER_COLOR.1, WATER_COLOR.2),
        TileType::Building => Color::rgb_u8(BUILDING_COLOR.0, BUILDING_COLOR.1, BUILDING_COLOR.2),
        TileType::Unsupported => Color::rgb_u8(
            UNSUPPORTED_COLOR.0,
            UNSUPPORTED_COLOR.1,
            UNSUPPORTED_COLOR.2,
        ),
    }
}
