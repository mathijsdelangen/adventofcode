use std::fs;
mod puzzle;

fn parse_input(input_file: &str) -> Vec<puzzle::Tile> {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let tile_descriptions : Vec<&str> = input.split("\r\n\r\n").collect();
    return tile_descriptions.iter().map(|d| puzzle::Tile::from_str(d)).collect();
}

fn determine(tiles: Vec<puzzle::Tile>) -> u128 {
    let mut result = 1u128;
    for tile_a in &tiles {
        let mut num_tiles_matching_border = 0;
        for tile_b in &tiles {
            if tile_a.id == tile_b.id {continue;}
            // Compare borders
            if puzzle::Tile::matches_border(tile_a, tile_b) {
                num_tiles_matching_border += 1;
            }
        }
        println!("Tile {} has {} matching borders", tile_a.id, num_tiles_matching_border);
        if num_tiles_matching_border == 2 {
            result *= (tile_a.id as u128);
        }
    }
    return result;
}

fn main() {
    let input_file = "assets/dec_20.in";

    println!("Solution: {}", determine(parse_input(input_file)) );
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_example(){
        let input_file = "assets/dec_20_example.in";
        assert_eq!(20899048083289, determine(parse_input(input_file)));
    }
}