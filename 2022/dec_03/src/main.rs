use std::fs;

struct Rucksack {
    compartment0: Vec<char>,
    compartment1: Vec<char>,
}

impl Rucksack {
    fn get_full_content(&self) -> Vec<char> {
        let mut content = Vec::new();
        for c in self.compartment0.iter() {
            content.push(*c);
        }
        
        for c in self.compartment1.iter() {
            content.push(*c);
        }
        return content;
    }
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

fn find_duplicate_item(compartment0: &Vec<char>, compartment1: &Vec<char>) -> Vec<char> {
    let mut comp0 = compartment0.to_vec();
    let mut comp1 = compartment1.to_vec();
    let mut duplicates = Vec::new();

    comp0.sort_by(|a, b| return compare_priorities(a, b));
    comp1.sort_by(|a, b| return compare_priorities(a, b));

    let mut index0 = 0;
    let mut index1 = 0;
    while index0 < comp0.len() && index1 < comp1.len() {
        let x = compare_priorities(&comp0[index0], &comp1[index1]);
        if x.is_eq() {
            duplicates.push(comp0[index0]);
            index0 += 1;
            index1 += 1;
        } else if x.is_le()
        {
            index0 += 1
        }
        else {
            index1 += 1
        }
    }
    return duplicates;
}

fn find_duplicate_of_three(r0: &Rucksack, r1: &Rucksack, r2: &Rucksack) -> char {

    let duplicates = find_duplicate_item(&r0.get_full_content(), &r1.get_full_content());
    let duplicate = find_duplicate_item(&duplicates, &r2.get_full_content());

    return duplicate[0];
}


fn first_solution(rucksacks: &Vec<Rucksack>) -> usize {
    let mut sum_of_priorities = 0;
    for rucksack in rucksacks {
        sum_of_priorities += get_priority(find_duplicate_item(&rucksack.compartment0, &rucksack. compartment1)[0]);
    }

    return sum_of_priorities;
}

fn second_solution(rucksacks: Vec<Rucksack>) -> usize {
    let mut sum_of_priorities = 0;
    for idx in (0..rucksacks.len()).step_by(3) {
        sum_of_priorities += get_priority(find_duplicate_of_three(&rucksacks[idx], &rucksacks[idx+1], &rucksacks[idx+2]));
        
    }

    return sum_of_priorities;
}

fn main() {
    let input_file = parse_input("assets/input.in");
    println!("Solution 1: {}", first_solution(&input_file));
    println!("Solution 2: {}", second_solution(input_file));
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
        assert_eq!(157, first_solution(&parse_input("assets/example.in")));
    }

    #[test]
    fn validate_example_2() {
        assert_eq!(70, second_solution(parse_input("assets/example.in")));
    }
}
