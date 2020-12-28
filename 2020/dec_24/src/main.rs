use std::fs;
mod floor;

fn parse_input(input_file: &str) -> Vec<floor::Tile> {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let tiles_to_flip : Vec<floor::Tile> = input.split("\r\n")
                                        .map(|l| floor::Tile::from_str(l))
                                        .collect();


    return tiles_to_flip;
}

fn count_black_tiles(tiles: Vec<floor::Tile>) -> usize {
    return tiles.len();    
}

fn black_tiles(tiles_to_flip: Vec<floor::Tile>) -> Vec<floor::Tile> {
    let mut unique_tiles = tiles_to_flip.to_owned();
    unique_tiles.dedup();

    let mut black_tiles = Vec::new();

    for tile in unique_tiles {
        let flips = tiles_to_flip.iter().filter(|&t| *t == tile).count();
        if flips % 2 == 1 {
            black_tiles.push(tile);
        }
    }
        
    return black_tiles;
}

fn count_black_neighbors(tile: floor::Tile, tiles: &Vec<floor::Tile>) -> u32 {
    let mut count = 0;
    
    let mut t = tile.to_owned(); t.east+=1; t.rescale();
    if tiles.contains(&t) { count += 1}
    let mut t = tile.to_owned(); t.east-=1; t.rescale();
    if tiles.contains(&t) { count += 1}
    let mut t = tile.to_owned(); t.north_east+=1; t.rescale();
    if tiles.contains(&t) { count += 1}
    let mut t = tile.to_owned(); t.north_east-=1; t.rescale();
    if tiles.contains(&t) { count += 1}
    let mut t = tile.to_owned(); t.south_east+=1; t.rescale();
    if tiles.contains(&t) { count += 1}
    let mut t = tile.to_owned(); t.south_east-=1; t.rescale();
    if tiles.contains(&t) { count += 1}

    return count;
}

fn has_zero_or_more_than_2_black_neighbors(tile: floor::Tile, tiles: &Vec<floor::Tile>) -> bool {

    let count = count_black_neighbors(tile, tiles);
    return count == 0 || count > 2
}

fn determine_white_neighbors_to_flip(tile: floor::Tile, tiles: &Vec<floor::Tile>) -> Vec<floor::Tile> {

    let mut result = Vec::new();

    let mut t = tile.to_owned(); t.east+=1; t.rescale();
    if !tiles.contains(&t) && count_black_neighbors(t, tiles) == 2 { result.push(t)}
    let mut t = tile.to_owned(); t.east-=1;  t.rescale();
    if !tiles.contains(&t) && count_black_neighbors(t, tiles) == 2 { result.push(t)}
    let mut t = tile.to_owned(); t.north_east+=1;  t.rescale();
    if !tiles.contains(&t) && count_black_neighbors(t, tiles) == 2 { result.push(t)}
    let mut t = tile.to_owned(); t.north_east-=1;  t.rescale();
    if !tiles.contains(&t) && count_black_neighbors(t, tiles) == 2 { result.push(t)}
    let mut t = tile.to_owned(); t.south_east+=1;  t.rescale();
    if !tiles.contains(&t) && count_black_neighbors(t, tiles) == 2 { result.push(t)}
    let mut t = tile.to_owned(); t.south_east-=1;  t.rescale();
    if !tiles.contains(&t) && count_black_neighbors(t, tiles) == 2 { result.push(t)}

    return result;
}

fn determine_new_tiles(tiles: Vec<floor::Tile>) -> Vec<floor::Tile> {
    let mut result = Vec::new();

    // Determine black tiles to stay
    for tile in &tiles {
        if !has_zero_or_more_than_2_black_neighbors(*tile, &tiles) {
            result.push(*tile);
        }
        result.extend(determine_white_neighbors_to_flip(*tile, &tiles));
    }
    result.sort();
    result.dedup();

    return result;
}

fn main() {
    let input_file = "assets/dec_24.in";
    let mut tiles = black_tiles(parse_input(input_file));

    for day in 0..100 {
        println!{"Day {}", day+1};
        tiles = determine_new_tiles(tiles);
    }

    println!("Solution: {}", count_black_tiles(tiles));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_example() {
        let input_file = "assets/dec_24_example.in";
        let starting_tiles = black_tiles(parse_input(input_file));
        assert_eq!(10, count_black_tiles(starting_tiles));
    }

    #[test]
    fn validate_example2() {
        let input_file = "assets/dec_24_example.in";
        let mut tiles = black_tiles(parse_input(input_file));
        for _ in 0..100 {
            tiles = determine_new_tiles(tiles);
        }
        assert_eq!(2208, count_black_tiles(tiles));
    }

    #[test]
    fn validate_example2_day_1_() {
        let input_file = "assets/dec_24_example.in";
        let mut tiles = black_tiles(parse_input(input_file));
        for _ in 0..1 {
            tiles = determine_new_tiles(tiles);
        }
        assert_eq!(15, count_black_tiles(tiles));
    }

    #[test]
    fn validate_example2_day_2_() {
        let input_file = "assets/dec_24_example.in";
        let mut tiles = black_tiles(parse_input(input_file));
        for _ in 0..2 {
            tiles = determine_new_tiles(tiles);
        }
        assert_eq!(12, count_black_tiles(tiles));
    }

    #[test]
    fn validate_example2_day_20_() {
        let input_file = "assets/dec_24_example.in";
        let mut tiles = black_tiles(parse_input(input_file));
        for _ in 0..20 {
            tiles = determine_new_tiles(tiles);
        }
        assert_eq!(132, count_black_tiles(tiles));
    }
}