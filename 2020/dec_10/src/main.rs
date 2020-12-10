use std::fs;

fn parse_input(input_file: &str) -> Vec<u64> {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let input_lines : Vec<u64> = input.split("\n")
                                        .map(|v| v.trim())
                                        .map(|v| v.parse::<u64>().unwrap())
                                        .collect();


    return input_lines;
}

fn find_differences(input: &Vec<u64>) -> (usize,usize) {

    let mut sorted_input = input.to_owned();
    sorted_input.sort();

    // Add 0 and max+3
    sorted_input.insert(0,0);
    sorted_input.push(sorted_input[sorted_input.len()-1] + 3);

    let jolt_1_differences = sorted_input.windows(2).filter(|&a| a[0]+1 == a[1]).count();
    let jolt_3_differences = sorted_input.windows(2).filter(|&a| a[0]+3 == a[1]).count();

    return (jolt_1_differences,jolt_3_differences);
}

fn find_arrangements_from(from_val: u64, values: &[u64]) -> u64 {
    if values.is_empty() { return 1; }

    let mut nr_arrangements = 0;
    for i in 0..3 {
        if i < values.len() && values[i] <= from_val + 3 {
            nr_arrangements += find_arrangements_from(values[i], &values[i+1..]);
        }       
    }
    return nr_arrangements;
}

fn find_arrangements(input: &Vec<u64>) -> u64 {
    let mut sorted_input = input.to_owned();
    sorted_input.sort();

    // Add max+3
    sorted_input.push(sorted_input[sorted_input.len()-1] + 3);

    return find_arrangements_from(0, &sorted_input);
}

fn main() {
    let input_file = "assets/dec_10.in";
    let values = parse_input(input_file);

    let (jolt_1_differences, jolt_3_differences) = find_differences(&values);
    println!("Solution 1: {}", jolt_1_differences * jolt_3_differences);

    println!("Solution 2: {}", find_arrangements(&values));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_find_differences_examples() {
        assert_eq!((7,5), find_differences(&parse_input("assets/dec_10_example1.in")));
        assert_eq!((22,10), find_differences(&parse_input("assets/dec_10_example2.in")));
    }

    #[test]
    fn validate_find_arrangements_examples() {
        assert_eq!(8, find_arrangements(&parse_input("assets/dec_10_example1.in")));
        assert_eq!(19208, find_arrangements(&parse_input("assets/dec_10_example2.in")));
    }

    #[test]
    fn validate_find_arrangements_from() {
        assert_eq!(1, find_arrangements_from(0, &[3,6,9,12,15,18]));
        assert_eq!(2, find_arrangements_from(0, &[3,6,9,12,15,17,18]));
        assert_eq!(2, find_arrangements_from(0, &[3,4,6,9,12,15,18]));
    }
}