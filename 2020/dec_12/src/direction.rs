
pub struct Location {
    pub north: i32,
    pub east: i32,
}

impl Location {
    pub fn new(north: i32, east: i32) -> Location {
        Location {
            north: north,
            east: east
        }
    }

    pub fn rotate(&self, instruction: &Instruction) -> Location {
        let nr_turns = instruction.units / 90;
        let mut location = Location::new(self.north, self.east);
        for _ in 0..nr_turns {
            match instruction.direction {
                'R' => {
                    let temp = location.east;
                    location.east = location.north;
                    location.north = -temp;
                },
                'L' => {
                    let temp = location.east;
                    location.east = -location.north;
                    location.north = temp;
                },
                _   => panic!("Heading unknown"),
            }
        }
        return location;
    }

    pub fn calculate_new_location(&self, instruction: &Instruction) -> Location {
        match instruction.direction {
            'N' => return Location::new(self.north+instruction.units, self.east),
            'S' => return Location::new(self.north-instruction.units, self.east),
            'E' => return Location::new(self.north, self.east+instruction.units),
            'W' => return Location::new(self.north, self.east-instruction.units),
            'L' => return self.rotate(&instruction),
            'R' => return self.rotate(&instruction),
            'F' => panic!("'F' is not a valid instruction anymore"),
            _   => panic!("Direction unknown"),
        }
    }
}

pub struct Instruction {
    pub direction: char,
    pub units: i32,
}

impl Instruction {
    pub fn new(direction: char, units: i32) -> Instruction {
        Instruction {
            direction: direction,
            units: units,
        }
    }

    pub fn new_from_string(direction: &str, units: &str) -> Instruction {
        Instruction {
            direction: direction.chars().next().unwrap(), //direction,
            units: units.parse::<i32>().unwrap(), //units,
        }
    }
}
