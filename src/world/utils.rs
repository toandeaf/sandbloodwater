use crate::world::component::TileType;
use std::fs::File;
use std::io::{BufRead, BufReader};

const TSV_DELIMITER: char = '\t';

pub fn load_map_config_from_file() -> Vec<Vec<TileType>> {
    // TODO use asset server?
    let file = File::open("assets/map.tsv").unwrap();

    let reader = BufReader::new(file);

    let mut tile_rows: Vec<Vec<TileType>> = vec![];

    // We read these in from top left of the file, the opposite way we'd be rendering them.
    for line_res in reader.lines() {
        let mut characters: Vec<TileType> = vec![];
        for character in line_res.unwrap().split(TSV_DELIMITER) {
            characters.push(character.parse::<TileType>().unwrap());
        }
        tile_rows.push(characters);
    }

    // So reverse the order so we read in from x = 0 and y = 0;
    tile_rows.reverse();

    tile_rows
}
