use std::fs;

fn parse_input(input_file: &str) -> Vec<u64> {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let mut input_lines : Vec<u64> = input.split("\n")
                                        .map(|v| v.trim())
                                        .map(|v| v.parse::<u64>().unwrap())
                                        .collect();

    // Add 0 and (max+3) to list of values
    let max_value = *input_lines.iter().max().unwrap() + 3;
    input_lines.push(0);
    input_lines.push(max_value);

    return input_lines;
}

fn find_differences(input: &Vec<u64>) -> (usize,usize) {
    let mut sorted_input = input.to_owned();
    sorted_input.sort();

    let jolt_1_differences = sorted_input.windows(2).filter(|&a| a[0]+1 == a[1]).count();
    let jolt_3_differences = sorted_input.windows(2).filter(|&a| a[0]+3 == a[1]).count();

    return (jolt_1_differences,jolt_3_differences);
}

fn find_arrangements(input: &Vec<u64>) -> u64 {
    let max_nr_elements : usize = *(input.iter().max().unwrap()) as usize;
    let mut nr_arrangements = vec![0; max_nr_elements+3]; //  Add 3 to size so we do not have to check overflow in the loop below
    nr_arrangements[max_nr_elements] = 1;

    for i in (0..max_nr_elements).rev() {
        if input.contains(&(i as u64)) {
            nr_arrangements[i] = nr_arrangements[i+1] + nr_arrangements[i+2] + nr_arrangements[i+3];
        }
    }

    return nr_arrangements[0];
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
        assert_eq!(1, find_arrangements(&vec![0,3,6,9,12,15,18]));
        assert_eq!(2, find_arrangements(&vec![0,3,6,9,12,15,17,18]));
        assert_eq!(2, find_arrangements(&vec![0,3,4,6,9,12,15,18]));
    }
}