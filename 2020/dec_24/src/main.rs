use std::fs;
mod floor;

fn parse_input(input_file: &str) -> Vec<floor::Tile> {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let tiles_to_flip : Vec<floor::Tile> = input.split("\r\n")
                                        .map(|l| floor::Tile::from_str(l))
                                        .collect();


    return tiles_to_flip;
}

fn count_black_tiles(tiles_to_flip: Vec<floor::Tile>) -> u32 {
    let mut unique_tiles = tiles_to_flip.to_owned();
    unique_tiles.dedup();

    let mut black_tiles = 0;
    for tile in unique_tiles {
        println!("Tile: {:?}", tile);
        let flips = tiles_to_flip.iter().filter(|&t| *t == tile).count();
        println!("Flip {} times", flips);
        if flips % 2 == 1 {
            black_tiles = black_tiles + 1;
        }
    }


    return black_tiles;
}

fn main() {
    let input_file = "assets/dec_24.in";

    println!("Solution : {}", count_black_tiles(parse_input(input_file)));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_example() {
        let input_file = "assets/dec_24_example.in";
        assert_eq!(10, count_black_tiles(parse_input(input_file)));
    }
}