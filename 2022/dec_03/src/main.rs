use std::fs;

struct Rucksack {
    compartment0: Vec<char>,
    compartment1: Vec<char>,
}

fn create_from_line(line: &str) -> Rucksack {
    let chars: Vec<char> = line.chars().collect();

    return Rucksack {
        compartment0: chars[..chars.len() / 2].to_vec(),
        compartment1: chars[chars.len() / 2..].to_vec(),
    };
}
fn compare_priorities(a: &char, b: &char) -> std::cmp::Ordering {
    return get_priority(*a).cmp(&get_priority(*b));
}
fn get_priority(c: char) -> usize {
    if c.is_lowercase() {
        return c as usize - 'a' as usize + 1;
    } else {
        return c as usize - 'A' as usize + 27;
    }
}

fn parse_input(input_file: &str) -> Vec<Rucksack> {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let rucksacks = input
        .split("\n")
        .map(|line| create_from_line(line.trim()))
        .collect();

    return rucksacks;
}

fn first_solution(rucksacks: Vec<Rucksack>) -> usize {
    let mut sum_of_priorities = 0;
    for rucksack in rucksacks {
        let mut comp0 = rucksack.compartment0;
        let mut comp1 = rucksack.compartment1;

        comp0.sort_by(|a, b| return compare_priorities(a, b));
        comp1.sort_by(|a, b| return compare_priorities(a, b));

        let mut index0 = 0;
        let mut index1 = 0;
        while index0 < comp0.len() && index1 < comp1.len() {
            let x = compare_priorities(&comp0[index0], &comp1[index1]);
            if x.is_eq() {
                sum_of_priorities += get_priority(comp0[index0]);
                break;
            } else if x.is_le()
            {
                index0 += 1
            }
            else {
                index1 += 1
            }
        }
    }

    return sum_of_priorities;
}

fn main() {
    let input_file = parse_input("assets/input.in");
    println!("Solution 1: {}", first_solution(input_file));
    //println!("Solution 2: {}", second_solution(input_file));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_create_from_line() {
        let rucksack = create_from_line("abcdef");
        assert_eq!(3, rucksack.compartment0.len());
        assert_eq!(3, rucksack.compartment1.len());
        assert_eq!('a', rucksack.compartment0[0]);
        assert_eq!('d', rucksack.compartment1[0]);
        assert_eq!('f', rucksack.compartment1[2]);
    }

    #[test]
    fn validate_get_priority() {
        assert_eq!(1, get_priority('a'));
        assert_eq!(26, get_priority('z'));
        assert_eq!(27, get_priority('A'));
        assert_eq!(52, get_priority('Z'));
    }

    #[test]
    fn validate_example_1() {
        assert_eq!(157, first_solution(parse_input("assets/example.in")));
    }

    #[test]
    fn validate_example_2() {
        //assert_eq!(12, second_solution(parse_input("assets/example.in")));
    }
}
