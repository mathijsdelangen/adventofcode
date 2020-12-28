use std::fmt;

#[derive(PartialEq, PartialOrd)]
#[derive(Clone, Copy)]
pub struct Tile {
    pub east: i32, // vs west
    pub north_east: i32, // vs south west
    pub south_east: i32, // vs north west
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tile")
         .field("east", &self.east)
         .field("north east", &self.north_east)
         .field("south east", &self.south_east)
         .finish()
    }
}

impl Tile {
    pub fn from_str(description: &str) -> Tile {
        let mut east = 0;
        let mut north_east = 0;
        let mut south_east = 0;

        let mut i = 0;
        while i != description.len() {
            let c = description.chars().nth(i).unwrap();
            match c {
                'e' => east += 1, 
                'w' => east -= 1,
                's' => {
                    let c2 = description.chars().nth(i+1).unwrap();
                    match c2 {
                        'e' => { south_east += 1; i += 1 }
                        'w' => { north_east -= 1; i += 1 }
                        _ => panic!("Char {} unknown", c2)
                    }
                }
                'n' => {
                    let c2 = description.chars().nth(i+1).unwrap();
                    match c2 {
                        'e' => { north_east += 1; i += 1 }
                        'w' => { south_east -= 1; i += 1 }
                        _ => panic!("Char {} unknown", c2)
                    }
                }
                _ => panic!("Char {} unknown", c)
            }

            i += 1;
        }

        // Postprocess
        if north_east > 0 && south_east > 0 {
            let min_val = std::cmp::min(north_east, south_east);
            north_east -= min_val;
            south_east -= min_val;
            east += min_val;
        }

        if north_east < 0 && south_east < 0 {
            let min_val = std::cmp::max(north_east, south_east);
            north_east -= min_val;
            south_east -= min_val;
            east += min_val;
        }

        if east > 0 {
            if north_east < 0 {
                let min_val = std::cmp::min((north_east as i32).abs(), east);
                east -= min_val;
                south_east += min_val;

                north_east += min_val;
            } 
            if south_east < 0 {
                let min_val = std::cmp::min((south_east as i32).abs(), east);
                east -= min_val;
                north_east += min_val;

                south_east += min_val;
            }
        }

        if east < 0 {
            if north_east > 0 {
                let min_val = std::cmp::min(north_east, (east as i32).abs());
                east += min_val;
                south_east -= min_val;

                north_east -= min_val;

            } 
            if south_east > 0 {
                let min_val = std::cmp::min(south_east, (east as i32).abs());
                east += min_val;
                north_east -= min_val;

                south_east -= min_val;
            }
        }
   
        return Tile { east:east, north_east:north_east, south_east:south_east };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_equal() {
        assert_eq!(Tile::from_str("e"), Tile::from_str("nese"));
        assert_eq!(Tile::from_str("w"), Tile::from_str("nwsw"));
        assert_eq!(Tile::from_str("nenw"), Tile::from_str("nwne"));
        assert_eq!(Tile::from_str("sesw"), Tile::from_str("swse"));

        assert_eq!(Tile::from_str("eee"), Tile::from_str("nenesenesese"));
        assert_eq!(Tile::from_str("senenwnw"), Tile::from_str("nwne"));
        
        assert_eq!(Tile::from_str("esew"), Tile::from_str("se"));
        assert_eq!(Tile::from_str("nwwswee"), Tile::from_str(""));

        assert_eq!(Tile::from_str("wwwne"), Tile::from_str("wwnw"));
        assert_eq!(Tile::from_str("wwwse"), Tile::from_str("wwsw"));

        assert_eq!(Tile::from_str("eeenw"), Tile::from_str("eene"));
        assert_eq!(Tile::from_str("eeesw"), Tile::from_str("eese"));
        
    }
}