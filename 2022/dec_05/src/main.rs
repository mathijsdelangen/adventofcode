use std::fs;

#[derive(Debug, Clone)]
struct Stack {
    crates: Vec<char>,
}
impl Stack {
    fn add(&mut self, c: char) -> () {
        self.crates.push(c);
    }
    fn remove(&mut self) -> char {
        return self.crates.pop().unwrap();
    }
    fn top(&self) -> char {
        if let Some(val) = self.crates.last() {
            return *val;
        } else {
            return ' ';
        }
    }
}

#[derive(Debug, Clone)]
struct Supplies {
    stacks: Vec<Stack>,
}
#[derive(Debug, Clone)]
struct Move {
    num_items: usize,
    from: usize,
    to: usize,
}

fn parse_move_from_line(line: &str) -> Move {
    let parts: Vec<&str> = line.split(" ").map(|p| p.trim()).collect();
    return Move {
        num_items: parts[1].parse::<usize>().unwrap(),
        from: parts[3].parse::<usize>().unwrap(),
        to: parts[5].parse::<usize>().unwrap(),
    };
}

fn parse_input_moves(lines: &str) -> Vec<Move> {
    return lines
        .split("\n")
        .map(|m| parse_move_from_line(&m))
        .collect();
}

fn parse_input_supplies(lines: &str) -> Supplies {
    // Initialize
    let mut supplies = Supplies {
        stacks: vec![Stack { crates: Vec::new() }; 9],
    };

    let mut filtered_lines: Vec<&str> = lines.split("\n").collect();
    filtered_lines.remove(filtered_lines.len() - 1);
    for line in filtered_lines.iter().rev() {
        for i in 0..=9 {
            match line.chars().nth(i * 4 + 1) {
                Some(c) => {
                    if c.is_alphabetic() {
                        supplies.stacks[i].add(c);
                    }
                }
                None => (),
            }
        }
    }

    return supplies;
}

fn parse_input(input_file: &str) -> (Supplies, Vec<Move>) {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let lines: Vec<&str> = input.split("\r\n\r\n").collect();

    println! {"{:?}", lines};
    let supplies = parse_input_supplies(lines[0]);
    let moves = parse_input_moves(lines[1]);

    return (supplies, moves);
}

fn solution(assignment: &(Supplies, Vec<Move>)) -> String {
    let mut supplies = assignment.0.clone();
    let moves = assignment.1.to_vec();
    for m in moves {
        let mut crates_to_add = Vec::new();

        for _ in 0..m.num_items {
            crates_to_add.push(supplies.stacks[m.from - 1].remove());
        }
        crates_to_add.reverse();
        for supply_crate in crates_to_add {
            supplies.stacks[m.to - 1].add(supply_crate);
        }
    }

    let mut result = Vec::new();
    for s in supplies.stacks {
        let c = s.top();
        if c.is_alphabetic() {
            result.push(c);
        }
    }

    return result.iter().collect();
}

fn main() {
    let input_file = parse_input("assets/input.in");
    //println!("Solution 1: {:?}", first_solution(&input_file));
    println!("Solution 2: {:?}", solution(&input_file));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_parse_move() {
        let move_text = "move 11 from 1 to 7";
        assert_eq!(11, parse_move_from_line(move_text).num_items);
        assert_eq!(1, parse_move_from_line(move_text).from);
        assert_eq!(7, parse_move_from_line(move_text).to);
    }

    #[test]
    fn validate_example_1() {
        //assert_eq!("CMZ", first_solution(&parse_input("assets/example.in")));
    }

    #[test]
    fn validate_example_2() {
        assert_eq!("MCD", solution(&parse_input("assets/example.in")));
    }
}
