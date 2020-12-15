use std::fs;

fn parse_line(line: &str) -> Vec<u32> {
    return line.split(",")
                .map(|v| v.trim())
                .map(|v| v.parse::<u32>().unwrap())
                .collect();
}
fn parse_input(input_file: &str) -> Vec<u32> {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    return parse_line(&input);
}

fn calculate_nth_number(starting_values : Vec<u32>, iterations: usize) -> u32 {  
    let mut last_time_spoken = vec![0;30000000];
    let mut last_number_spoken = 0;
    
    for turn in 1..=starting_values.len() {
        last_time_spoken[starting_values[turn-1] as usize] = turn;
        last_number_spoken = starting_values[turn-1];
    }

    for turn in starting_values.len()+1..=iterations {
        let last_occurence = last_time_spoken[last_number_spoken as usize];
        last_time_spoken[last_number_spoken as usize] = turn - 1;
        match last_occurence {
            0 => last_number_spoken = 0,
            _ => last_number_spoken = (turn - 1 - last_occurence) as u32,
        }
    }
    return last_number_spoken;
}

fn main() {
    let input_file = "assets/dec_15.in";
    println!("Solution: {}", calculate_nth_number(parse_input(input_file), 30000000));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_examples() {
        assert_eq!(436,  calculate_nth_number(parse_line("0,3,6"), 2020));
        assert_eq!(1,    calculate_nth_number(parse_line("1,3,2"), 2020));
        assert_eq!(10,   calculate_nth_number(parse_line("2,1,3"), 2020));
        assert_eq!(27,   calculate_nth_number(parse_line("1,2,3"), 2020));
        assert_eq!(78,   calculate_nth_number(parse_line("2,3,1"), 2020));
        assert_eq!(438,  calculate_nth_number(parse_line("3,2,1"), 2020));
        assert_eq!(1836, calculate_nth_number(parse_line("3,1,2"), 2020));
    }

    #[test]
    fn validate_examples2() {
        assert_eq!(175594,  calculate_nth_number(parse_line("0,3,6"), 30000000));
        assert_eq!(2578,    calculate_nth_number(parse_line("1,3,2"), 30000000));
        assert_eq!(3544142, calculate_nth_number(parse_line("2,1,3"), 30000000));
        assert_eq!(261214,  calculate_nth_number(parse_line("1,2,3"), 30000000));
        assert_eq!(6895259, calculate_nth_number(parse_line("2,3,1"), 30000000));
        assert_eq!(18,      calculate_nth_number(parse_line("3,2,1"), 30000000));
        assert_eq!(362,     calculate_nth_number(parse_line("3,1,2"), 30000000));
    }
}