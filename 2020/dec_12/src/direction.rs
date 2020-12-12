
pub struct Location {
    pub north: i32,
    pub east: i32,
    heading: char,
}

impl Location {
    pub fn new(north: i32, east: i32, heading: char) -> Location {
        Location {
            north: north,
            east: east,
            heading: heading,
        }
    }

    pub fn calculate_new_heading(&self, instruction: &Instruction) -> char {
        println!("Rotate {} with {} degrees", instruction.direction, instruction.units);
        let turn = instruction.direction;
        let nr_turns = instruction.units / 90;
        println!(" so rotate {} times", nr_turns);
        let mut heading = self.heading;
        for _ in 0..nr_turns {
            match heading {
                'N' => if turn == 'L' {heading = 'W'} else {heading = 'E'},
                'E' => if turn == 'L' {heading = 'N'} else {heading = 'S'},
                'S' => if turn == 'L' {heading = 'E'} else {heading = 'W'},
                'W' => if turn == 'L' {heading = 'S'} else {heading = 'N'},
                _   => panic!("Heading unknown"),
            }
        }
        return heading
    }

    pub fn calculate_new_location(&self, instruction: &Instruction) -> Location {
        match instruction.direction {
            'N' => return Location::new(self.north+instruction.units, self.east, self.heading),
            'S' => return Location::new(self.north-instruction.units, self.east, self.heading),
            'E' => return Location::new(self.north, self.east+instruction.units, self.heading),
            'W' => return Location::new(self.north, self.east-instruction.units, self.heading),
            'L' => return Location::new(self.north, self.east, self.calculate_new_heading(&instruction)),
            'R' => return Location::new(self.north, self.east, self.calculate_new_heading(&instruction)),
            'F' => return self.calculate_new_location(&Instruction::new(self.heading, instruction.units)),
            _   => panic!("Direction unknown"),
        }
    }
}

pub struct Instruction {
    direction: char,
    units: i32,
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
