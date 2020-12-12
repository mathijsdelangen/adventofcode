use std::fs;
mod direction;

fn parse_input(input_file: &str) -> Vec<direction::Instruction> {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let input_lines : Vec<direction::Instruction> = input.split("\n")
                                        .map(|v| v.trim())
                                        .map(|v| direction::Instruction::new_from_string(&v[0..1], &v[1..]))
                                        .collect();
    return input_lines;
}

fn find_manhattan_distance(instructions: Vec<direction::Instruction>) -> i32 {
    
    let mut ship_location = direction::Location::new(0,0);
    let mut waypoint_location = direction::Location::new(1,10);
    for instruction in instructions {
        match instruction.direction {
            'F' => {
                ship_location.north += waypoint_location.north * instruction.units;
                ship_location.east += waypoint_location.east * instruction.units;
            }
            _ => waypoint_location = waypoint_location.calculate_new_location(&instruction),
        }
    }
    return ship_location.north.abs() + ship_location.east.abs();
}

fn main() {
    let input_file = "assets/dec_12.in";

    println!("Solution 2: {}", find_manhattan_distance(parse_input(input_file)));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_example() {
        assert_eq!(286, find_manhattan_distance(parse_input("assets/dec_12_example.in")));
        assert_eq!(89984, find_manhattan_distance(parse_input("assets/dec_12.in")));
    }
}