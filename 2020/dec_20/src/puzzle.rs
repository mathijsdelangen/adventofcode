#[derive(Clone)]
pub struct Tile {
    pub id: u32,
    pub tile_description: Vec<String>,
}

impl Tile {
    pub fn from_str(description: &str) -> Tile {
        let desc : Vec<&str> = description.split("\r\n").collect();
        let id = desc[0].replace("Tile ", "").replace(":", "").parse::<u32>().unwrap();
        let mut tile_description = Vec::new();
        for desc in &desc[1..] {
            tile_description.push(desc.to_string());
        }
       
        Tile { id: id, tile_description: tile_description }
    }

    // Border order: right, bottom, left, top
    // Bottom en top border are described left to right
    // Left and right border are described top to bottom
    pub fn get_borders(&self) -> Vec<String> {
        let mut borders = Vec::new();
        {
            let mut left_border = "".to_string();
            let mut right_border = "".to_string();
            for i in 0..self.tile_description.len() {
                left_border += &self.tile_description[i].chars().nth(0).unwrap().to_string();
                right_border += &self.tile_description[i].chars().nth(self.tile_description[i].len()-1).unwrap().to_string();
            }
            borders.push(right_border);
            borders.push(self.tile_description[self.tile_description.len()-1].chars().collect()); // Bottom row
            borders.push(left_border);
        }
        borders.push(self.tile_description[0].chars().collect()); // Top row

        return borders;
    }

    pub fn flip_and_rotate_back(&mut self) {
        self.flip();
        self.rotate();
        self.rotate();
    }

    // Rotates tile by 90 degrees clock wise
    pub fn rotate(&mut self) {
        let mut new_tile_description : Vec<String> = vec!["".to_string();self.tile_description.len()];
        let mut current_tile_description = self.tile_description.to_owned();
        current_tile_description.reverse();
        for (_,line) in current_tile_description.iter().enumerate() {
            for (c_idx,c) in line.chars().enumerate() {
                new_tile_description[c_idx] += &c.to_string();
            }
        }
        self.tile_description = new_tile_description;
    }
    // Rotates tile by 90 degrees counter clockwise
    pub fn rotate_back(&mut self) {
        let mut new_tile_description : Vec<String> = vec!["".to_string();self.tile_description.len()];
        let mut current_tile_description = self.tile_description.to_owned();
        
        for (_,line) in current_tile_description.iter().enumerate() {
            for (c_idx,c) in line.chars().enumerate() {
                new_tile_description[c_idx] += &c.to_string();
            }
        }
        self.tile_description = new_tile_description;
    }


    pub fn flip(&mut self) {
        let mut new_tile_description = self.tile_description.to_owned();
        new_tile_description.reverse();
        self.tile_description = new_tile_description;
    }

    pub fn matches_border(tile_a: &Tile, tile_b: &Tile) -> bool {
        let borders = &tile_a.get_borders();
        for b in borders {
            if tile_b.get_borders().contains(&b) { return true }
            if tile_b.get_borders().contains(&b.chars().rev().collect()) { return true }
        }
        return false;
    }

    pub fn common_border(tile_a: &Tile, tile_b: &Tile) -> String {
        let borders = tile_a.get_borders().to_owned();
        for b in borders {
            if tile_b.get_borders().contains(&b) { return b }
            if tile_b.get_borders().contains(&b.chars().rev().collect()) { return b }
        }
        panic!("No common border found");
    }

    // Returns which border matches between the tiles
    // Rotation is right,bottom,left,top
    // (0,1) -> left border for tile_a, bottom border for tile_b
    pub fn find_common_border(tile_a: &Tile, tile_b: &Tile) -> (usize,usize) {

        let common_border = Tile::common_border(tile_a, tile_b);
        let mut common_border_reverse = common_border.to_owned();
        common_border_reverse = common_border_reverse.chars().rev().collect();
        let border_id_tile_a = tile_a.get_borders().iter().position(|x| *x == common_border || *x == common_border_reverse).unwrap();
        let border_id_tile_b = tile_b.get_borders().iter().position(|x| *x == common_border || *x == common_border_reverse).unwrap();
        
        return (border_id_tile_a,border_id_tile_b);
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
        
        assert_eq!(false, Tile::matches_border(&tile_a, &tile_i));
        assert_eq!(false, Tile::matches_border(&tile_b, &tile_i));
        assert_eq!(false, Tile::matches_border(&tile_e, &tile_i));
        
        assert_eq!(true, Tile::matches_border(&tile_h, &tile_i));
    }

    #[test]
    fn validate_get_borders() {
        let tile = Tile::from_str("Tile 1:\r\n###\r\n...\r\n...");
        assert_eq!(String::from("#.."), tile.get_borders()[0]);
        assert_eq!(String::from("..."), tile.get_borders()[1]);
        assert_eq!(String::from("#.."), tile.get_borders()[2]);
        assert_eq!(String::from("###"), tile.get_borders()[3]);
    }
    #[test]
    fn validate_rotate() {
        let mut tile = Tile::from_str("Tile 1:\r\n###\r\n...\r\n...");
        assert_eq!(String::from("#.."), tile.get_borders()[0]);
        assert_eq!(String::from("..."), tile.get_borders()[1]);
        assert_eq!(String::from("#.."), tile.get_borders()[2]);
        assert_eq!(String::from("###"), tile.get_borders()[3]);
        
        tile.rotate();
        assert_eq!(vec!["..#","..#","..#",], tile.tile_description);
        assert_eq!(String::from("###"), tile.get_borders()[0]);
        assert_eq!(String::from("..#"), tile.get_borders()[1]);
        assert_eq!(String::from("..."), tile.get_borders()[2]);
        assert_eq!(String::from("..#"), tile.get_borders()[3]);
    }

    #[test]
    fn validate_flip() {
        let mut tile = Tile::from_str("Tile 1:\r\n###\r\n...\r\n...");
        assert_eq!(String::from("#.."), tile.get_borders()[0]);
        assert_eq!(String::from("..."), tile.get_borders()[1]);
        assert_eq!(String::from("#.."), tile.get_borders()[2]);
        assert_eq!(String::from("###"), tile.get_borders()[3]);

        tile.flip();
        assert_eq!(vec!["...","...","###",], tile.tile_description);
        assert_eq!(String::from("..#"), tile.get_borders()[0]);
        assert_eq!(String::from("###"), tile.get_borders()[1]);
        assert_eq!(String::from("..#"), tile.get_borders()[2]);
        assert_eq!(String::from("..."), tile.get_borders()[3]);
    }
}