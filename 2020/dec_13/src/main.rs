use std::fs;

fn parse_input(input_file: &str) -> (u32, Vec<u32>) {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let input_lines : Vec<&str> = input.split("\n").map(|v| v.trim()).collect();

    let earliest_timestamp = input_lines[0].parse::<u32>().unwrap();
    let busses : Vec<u32> = input_lines[1].split(",")
                                        .filter(|&v| v != "x")
                                        .map(|v| v.parse::<u32>().unwrap())
                                        .collect();
    return (earliest_timestamp,busses)

}

fn determine_earliest_depart(timestamp: u32, busses: Vec<u32>) -> (u32, u32) {
    let mut result = 1024;
    for bus in busses {
        if bus - (timestamp % bus) < result - (timestamp % result) {
            result = bus;
        }
    }

    return (result, result - (timestamp % result) + timestamp);
}

fn main() {
    let input_file = "assets/dec_13.in";
    let (earliest_timestamp,busses) = parse_input(input_file);
    let (bus, departure_time) = determine_earliest_depart(earliest_timestamp, busses);
    println!("Solution 1: {}", (departure_time - earliest_timestamp) * bus);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_example1() {
        let input_file = "assets/dec_13_example.in";
        let (earliest_timestamp,busses) = parse_input(input_file);
        let (bus, departure_time) = determine_earliest_depart(earliest_timestamp, busses);

        assert_eq!(295, (departure_time - earliest_timestamp) * bus);
    }
}