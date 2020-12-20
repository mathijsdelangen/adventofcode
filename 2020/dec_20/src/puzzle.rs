pub struct Tile {
    pub id: u32,
    pub tile_description: String,
    pub borders: Vec<Vec<char>>,
}

impl Tile {
    pub fn from_str(description: &str) -> Tile {
        let desc : Vec<&str> = description.split("\r\n").collect();
        let id = desc[0].replace("Tile ", "").replace(":", "").parse::<u32>().unwrap();
        let tile_description = desc[1..].join("\r\n");
        let mut borders = Vec::new();
        // Top row
        borders.push(desc[1].chars().collect());
        {
            let mut left_border = Vec::new();
            let mut right_border = Vec::new();
            for i in 1..11 {
                left_border.push(desc[i].chars().nth(0).unwrap());
                right_border.push(desc[i].chars().nth(9).unwrap());
            }
            borders.push(left_border);
            borders.push(desc[10].chars().collect()); // Bottom row
            borders.push(right_border);
        }
        
        Tile { id: id, tile_description: tile_description, borders: borders}
    }

    pub fn matches_border(tile_a: &Tile, tile_b: &Tile) -> bool {
        for b in &tile_a.borders {
            if tile_b.borders.contains(&b) { return true }
            let mut b_reverse = b.to_owned();
            b_reverse.reverse();
            if tile_b.borders.contains(&b_reverse) { return true }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    /* Correct order of tiles from example
    1951    2311    3079
    2729    1427    2473
    2971    1489    1171

    */
    #[test]
    fn validate_matching_border(){
        let tile_a = Tile::from_str("Tile 1951:\r\n#.##...##.\r\n#.####...#\r\n.....#..##\r\n#...######\r\n.##.#....#\r\n.###.#####\r\n###.##.##.\r\n.###....#.\r\n..#.#..#.#\r\n#...##.#..");
        let tile_b = Tile::from_str("Tile 2311:\r\n..##.#..#.\r\n##..#.....\r\n#...##..#.\r\n####.#...#\r\n##.##.###.\r\n##...#.###\r\n.#.#.#..##\r\n..#....#..\r\n###...#.#.\r\n..###..###");
        
        let tile_e = Tile::from_str("Tile 1427:\r\n###.##.#..\r\n.#..#.##..\r\n.#.##.#..#\r\n#.#.#.##.#\r\n....#...##\r\n...##..##.\r\n...#.#####\r\n.#.####.#.\r\n..#..###.#\r\n..##.#..#.");
        let tile_h = Tile::from_str("Tile 1489:\r\n##.#.#....\r\n..##...#..\r\n.##..##...\r\n..#...#...\r\n#####...#.\r\n#..#.#.#.#\r\n...#.#.#..\r\n##.#...##.\r\n..##.##.##\r\n###.##.#..");
        let tile_i = Tile::from_str("Tile 1171:\r\n####...##.\r\n#..##.#..#\r\n##.#..#.#.\r\n.###.####.\r\n..###.####\r\n.##....##.\r\n.#...####.\r\n#.##.####.\r\n####..#...\r\n.....##...");
        
        assert_eq!(true, Tile::matches_border(&tile_a, &tile_a));
        assert_eq!(true, Tile::matches_border(&tile_a, &tile_b));
        assert_eq!(true, Tile::matches_border(&tile_b, &tile_a));
        
        assert_eq!(false, Tile::matches_border(&tile_b, &tile_i));
        assert_eq!(false, Tile::matches_border(&tile_a, &tile_i));
        
        assert_eq!(true, Tile::matches_border(&tile_h, &tile_i));
    }
}