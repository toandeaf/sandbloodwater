const NEW_LINE_DELIMITER: char = '\n';
const CSV_DELIMITER: char = ',';

pub fn load_map_config_from_file(map_string: &str) -> Vec<Vec<usize>> {
    let lines = map_string.split(NEW_LINE_DELIMITER);

    let mut tile_rows: Vec<Vec<usize>> = vec![];

    // We read these in from top left of the file, the opposite way we'd be rendering them.
    for line_res in lines {
        let mut characters: Vec<usize> = vec![];
        for character in line_res.split(CSV_DELIMITER) {
            let num = character.parse::<usize>();

            if let Ok(parsed_num) = num {
                characters.push(parsed_num);
            }
        }
        tile_rows.push(characters);
    }

    // So reverse the order so we read in from x = 0 and y = 0;
    tile_rows.reverse();

    tile_rows
}
