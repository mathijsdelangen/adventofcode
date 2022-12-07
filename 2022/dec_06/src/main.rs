use std::fs;

fn find_marker(input: &str, sequence_size: usize) -> usize {
    let elems: Vec<char> = input.chars().collect::<Vec<_>>();
    for i in sequence_size..elems.len() {
        let mut test = elems[i - sequence_size..i].to_vec();
        test.sort();
        test.dedup();
        if test.len() == sequence_size {
            return i;
        }
    }
    panic!("Could not find any marker")
}

fn first_solution(input: &str) -> usize {
    return find_marker(input, 4);
}

fn second_solution(input: &str) -> usize {
    return find_marker(input, 14);
}

fn main() {
    let input_file = "assets/input.in";
    let input_string =
        fs::read_to_string(input_file).expect("Something went wrong reading the file");
    println!("Solution 1: {:?}", first_solution(&input_string));
    println!("Solution 2: {:?}", second_solution(&input_string));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_examples_1() {
        let input_file = "assets/example.in";
        let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
        let examples: Vec<&str> = input.split("\n").map(|l| l.trim()).collect();
        
        assert_eq!(7, first_solution(examples[0]));
        assert_eq!(5, first_solution(examples[1]));
        assert_eq!(6, first_solution(examples[2]));
        assert_eq!(10, first_solution(examples[3]));
        assert_eq!(11, first_solution(examples[4]));
    }

    #[test]
    fn validate_examples_2() {
        let input_file = "assets/example.in";
        let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
        let examples: Vec<&str> = input.split("\n").map(|l| l.trim()).collect();
        
        assert_eq!(19, second_solution(examples[0]));
        assert_eq!(23, second_solution(examples[1]));
        assert_eq!(23, second_solution(examples[2]));
        assert_eq!(29, second_solution(examples[3]));
        assert_eq!(26, second_solution(examples[4]));
    }
}
