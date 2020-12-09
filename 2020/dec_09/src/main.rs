use std::fs;

fn parse_input(input_file: &str) -> Vec<u64> {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let input_lines : Vec<u64> = input.split("\n")
                                        .map(|v| v.trim())
                                        .map(|v| v.parse::<u64>().unwrap())
                                        .collect();

    return input_lines;
}

fn has_sum(value: u64, values_in_preamble: &[u64]) -> bool {
    for i in 0..values_in_preamble.len() {
        for j in i+1..values_in_preamble.len() {
            if value == values_in_preamble[i] + values_in_preamble[j] {
                return true;
            }
        }
    }
    return false;
}

fn find_incorrect_number(preamble_size: usize, values: &[u64]) -> u64 {
    for i in preamble_size..values.len() {
        if !has_sum(values[i], &values[i-preamble_size..i]) {
            return values[i];
        }
    }
    return 0;
}

fn find_contiguous_set(value: u64, values: &[u64]) -> (usize, usize) {
    for min in 0..values.len() {
        let mut sum = values[min];
        for max in min+1..values.len() {
            sum += values[max];
            if sum == value { return (min,max)}
            if sum > value { break; }
        }
    }
    return (0,0);
}

fn caluculate_weakness(value: u64, values: &[u64]) -> u64 {

    let (min,max) = find_contiguous_set(value, &values);
    let min_value = values[min..max+1].iter().min().unwrap();
    let max_value = values[min..max+1].iter().max().unwrap();
    return min_value+max_value;
}

fn main() {
    let input_file = "assets/dec_09.in";
    
    let values = parse_input(input_file);
    let incorrect_number = find_incorrect_number(25, &values);

    println!("Solution 1: {}", incorrect_number);
    println!("Solution 2: {}", caluculate_weakness(incorrect_number, &values));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_has_sum(){
        assert_eq!(true, has_sum(11, &[1,10]));
        assert_eq!(false, has_sum(11, &[10]));
        assert_eq!(false, has_sum(1, &[1,10]));
        assert_eq!(false, has_sum(10, &[1,10]));
        assert_eq!(true, has_sum(11, &[1,2,3,4,5,6,7,8,9,10]));
        assert_eq!(true, has_sum(13, &[1,2,3,4,5,6,7,8,9,10]));
        assert_eq!(true, has_sum(17, &[1,2,3,4,5,6,7,8,9,10]));
    }

    #[test]
    fn validate_find_contiguous_set(){
        assert_eq!((0,1), find_contiguous_set(11, &[1,10]));
        assert_eq!(1+10, caluculate_weakness(11, &[1,10]));

        assert_eq!((0,2), find_contiguous_set(6, &[1,2,3,10]));
        assert_eq!(1+3, caluculate_weakness(6, &[1,2,3,10]));

        assert_eq!((1,3), find_contiguous_set(15, &[1,2,3,10]));
        assert_eq!(2+10, caluculate_weakness(15, &[1,2,3,10]));
    }

    #[test]
    fn validate_example1(){
        assert_eq!(127, find_incorrect_number(5, &parse_input("assets/dec_09_example.in")));
    }

    #[test]
    fn validate_example2(){
        let values = parse_input("assets/dec_09_example.in");
        assert_eq!((2,5), find_contiguous_set(127, &values));
        assert_eq!(62, caluculate_weakness(127, &values));
    }
}