use std::fs;

// Return vec from bus_id -> added timestamp
fn parse_input(input_file: &str) -> Vec<(u128,u128)> {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let input_lines : Vec<&str> = input.split("\n").map(|v| v.trim()).collect();

    let busses : Vec<&str> = input_lines[1].split(",").collect();
    let mut busses_with_timestamps = Vec::new();
    for t in 0..busses.len() {
        match busses[t] {
            "x" => (),
            _   => busses_with_timestamps.push((busses[t].parse::<u128>().unwrap(), t as u128)),
        }
    }
    return busses_with_timestamps;
}

fn check_timestamp(current_time: u128, bus_with_timestamp: &(u128,u128)) -> bool {
    let bus = bus_with_timestamp.0;
    let timestamp = bus_with_timestamp.1;

    return (current_time + timestamp) % bus == 0;
}

fn update_incrementer(current_increment: u128, bus_id: u128) -> u128 {
    return current_increment * bus_id;
}

fn determine_earliest_timestap(busses: &Vec<(u128,u128)>) -> u128 {
    let mut current_timestamp = 0;
    let mut incrementer = 1;
    for bus in busses {
        // Find combined schedule with new bus
        loop {
            if check_timestamp(current_timestamp, bus) { 
                println!("Timestamp OK @ {} for {} (with t+{})", current_timestamp, bus.0, bus.1);
                incrementer = update_incrementer(incrementer, bus.0); 
                println!("Incrementer set to {}", incrementer);
                break; 
            }
            else {
                current_timestamp += incrementer
            }
        } 
    }

    return current_timestamp;
}

fn main() {
    let input_file = "assets/dec_13.in";
    println!("Solution: {}", determine_earliest_timestap(&parse_input(input_file)));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_example() {
        assert_eq!(1068781, determine_earliest_timestap(&parse_input("assets/dec_13_example.in")));
        assert_eq!(3417, determine_earliest_timestap(&parse_input("assets/dec_13_example1.in")));
        assert_eq!(754018, determine_earliest_timestap(&parse_input("assets/dec_13_example2.in")));
        assert_eq!(779210, determine_earliest_timestap(&parse_input("assets/dec_13_example3.in")));
        assert_eq!(1261476, determine_earliest_timestap(&parse_input("assets/dec_13_example4.in")));
        assert_eq!(1202161486, determine_earliest_timestap(&parse_input("assets/dec_13_example5.in")));
    }
}