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

fn find_manhattan_distance(directions: Vec<direction::Instruction>) -> i32 {
    
    let mut current_location = direction::Location::new(0,0,'E');
    for direction in directions {
        current_location = current_location.calculate_new_location(&direction);
    }
    return current_location.north.abs() + current_location.east.abs();
}

fn main() {
    let input_file = "assets/dec_12.in";

    println!("Solution 1: {}", find_manhattan_distance(parse_input(input_file)));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_example() {
        assert_eq!(25, find_manhattan_distance(parse_input("assets/dec_12_example.in")));
    }
}