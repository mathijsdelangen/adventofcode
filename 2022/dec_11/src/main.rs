use std::fmt;
use std::fs;

type MonkeyId = usize;
type ItemsThrown = Vec<Vec<usize>>;

enum Operator {
    Add,
    Mult,
}

struct Monkey {
    items: Vec<usize>,
    operator: Operator,
    input: Option<usize>, // If None, old is used as input too
    denominator: usize,
    monkey_if_test_true: MonkeyId,
    monkey_if_test_false: MonkeyId,
    inspections: usize,
}

impl Monkey {
    pub fn inspect(&mut self, nr_monkeys: usize, devide_by_three: bool, gcm: usize) -> ItemsThrown {
        let mut throw_items: ItemsThrown = vec![Vec::new(); nr_monkeys];

        for item in self.items.to_vec() {
            let new_item = if devide_by_three {self.do_operation(item) / 3} else {self.do_operation(item)} % gcm;

            let receiving_monkey = if self.test(new_item) {
                self.monkey_if_test_true
            } else {
                self.monkey_if_test_false
            };

            throw_items[receiving_monkey].push(new_item);
            self.inspections += 1;
        }

        self.items.clear();

        return throw_items;
    }

    pub fn do_operation(&self, item: usize) -> usize {
        let mut new_value = 0;
        match (&self.operator, &self.input) {
            (Operator::Add, None) => new_value = item + item,
            (Operator::Add, Some(value)) => new_value = item + value,
            (Operator::Mult, None) => new_value = item * item,
            (Operator::Mult, Some(value)) => new_value = item * value,
            _ => panic!("I'm lost"),
        }
        return new_value;
    }

    pub fn test(&self, item: usize) -> bool {
        return item % self.denominator == 0;
    }
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("inspections", &self.inspections)
            .finish()
    }
}

fn get_value_after_text(input_line: &str, prefix: &str) -> usize {
    let text: Vec<&str> = input_line.split(prefix).collect();
    return text[1].trim().parse::<usize>().unwrap();
}

fn get_values_after_text(input_line: &str, prefix: &str) -> Vec<usize> {
    let text: Vec<&str> = input_line.split(prefix).collect();
    return text[1]
        .split(",")
        .map(|i| i.trim())
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
}

//  Operation: new = old + 6
fn get_operator(input: &str) -> Operator {
    let parts: Vec<&str> = input.split(" ").map(|p| p.trim()).collect();
    match parts[4].chars().nth(0).unwrap() {
        '+' => return Operator::Add,
        '*' => return Operator::Mult,
        _ => panic!("Unknown operator {:?}", parts[1]),
    }
}

fn get_input(input: &str) -> Option<usize> {
    let parts: Vec<&str> = input.split(" ").map(|p| p.trim()).collect();
    match parts[5] {
        "old" => return None,
        value => return Some(value.parse::<usize>().unwrap()),
    }
}

fn parse_monkey(input: &str) -> Monkey {
    let lines: Vec<&str> = input.split("\n").map(|l| l.trim()).collect();

    return Monkey {
        items: get_values_after_text(lines[1], "items:"),
        operator: get_operator(lines[2]),
        input: get_input(lines[2]),
        denominator: get_value_after_text(lines[3], "by"),
        monkey_if_test_true: get_value_after_text(lines[4], "monkey"),
        monkey_if_test_false: get_value_after_text(lines[5], "monkey"),
        inspections: 0,
    };
}

fn parse_input(input_file: &str) -> Vec<Monkey> {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    return input
        .split("\r\n\r\n")
        .map(|l| parse_monkey(l.trim()))
        .collect::<Vec<_>>();
}

fn monkey_business(monkeys: &mut Vec<Monkey>, nr_rounds: usize, devide_by_three: bool) -> usize {
    let gcm = monkeys.iter().map(|m| m.denominator).product();

    let nr_monkeys = monkeys.len();
    for _ in 0..nr_rounds {
        for idx in 0..nr_monkeys {
            let items_thrown = monkeys[idx].inspect(nr_monkeys, devide_by_three, gcm);
            for (monkey_id, items) in items_thrown.iter().enumerate() {
                for item in items {
                    monkeys[monkey_id].items.push(*item);
                }
            }
        }
    }
    
    println!("Monkeys: {:?}", monkeys);
    
    // Get the two monkeys with the most monkey business
    let mut monkey_business_levels: Vec<usize> = monkeys.iter().map(|m| m.inspections).collect();
    monkey_business_levels.sort();
    monkey_business_levels.reverse();

    return monkey_business_levels[0] * monkey_business_levels[1];
}

fn main() {
    let mut monkeys_sol1 = parse_input("assets/input.in");
    println!("Solution 1: {:?}", monkey_business(&mut monkeys_sol1, 20, true));

    let mut monkeys_sol2 = parse_input("assets/input.in");
    println!("Solution 2: {:?}", monkey_business(&mut monkeys_sol2, 10000, false));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_example_1() {
        let mut input = parse_input("assets/example.in");
        assert_eq!(10605, monkey_business(&mut input, 20, true));
    }

    #[test]
    fn validate_example_2() {
        let mut input = parse_input("assets/example.in");
        assert_eq!(2713310158, monkey_business(&mut input, 10000, false));
    }
    
    #[test]
    fn validate_example_2_1_round() {
        let mut monkeys = parse_input("assets/example.in");
        let _ = monkey_business(&mut monkeys, 1, false);
        assert_eq!(2, monkeys[0].inspections);
        assert_eq!(4, monkeys[1].inspections);
        assert_eq!(3, monkeys[2].inspections);
        assert_eq!(6, monkeys[3].inspections);
    }

    #[test]
    fn validate_example_2_20_rounds() {
        let mut monkeys = parse_input("assets/example.in");
        let _ = monkey_business(&mut monkeys, 20, false);
        assert_eq!(99, monkeys[0].inspections);
        assert_eq!(97, monkeys[1].inspections);
        assert_eq!(8, monkeys[2].inspections);
        assert_eq!(103, monkeys[3].inspections);
    }

    #[test]
    fn validate_example_2_1000_rounds() {
        let mut monkeys = parse_input("assets/example.in");
        let _ = monkey_business(&mut monkeys, 1000, false);
        assert_eq!(5204, monkeys[0].inspections);
        assert_eq!(4792, monkeys[1].inspections);
        assert_eq!(199, monkeys[2].inspections);
        assert_eq!(5192, monkeys[3].inspections);
    }
}
