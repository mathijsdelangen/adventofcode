use std::fs;

type Instruction = (String, isize);

fn parse_line(line: &str) -> Instruction {
    let instruction: Vec<&str> = line.split(" ").collect();
    if instruction.len() == 1 {
        return (instruction[0].to_string(), 0);
    } else if instruction.len() == 2 {
        (
            instruction[0].to_string(),
            instruction[1].parse::<isize>().unwrap_or(0),
        )
    } else {
        panic!("Cannot parse '{:?}'", line);
    }
}

fn parse_input(input_file: &str) -> Vec<Instruction> {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    return input
        .split("\n")
        .map(|l| parse_line(l.trim()))
        .collect::<Vec<_>>();
}

fn calculate_individual_signal_strengths(instructions: &Vec<Instruction>) -> Vec<isize> {
    let mut cycle = 1;
    let mut reg = 1;
    let mut signal_strength: Vec<isize> = Vec::new();
    signal_strength.push(cycle * reg);

    for instruction in instructions {
        match instruction.0.as_str() {
            "addx" => {
                signal_strength.push(cycle * reg);
                cycle += 1;
                signal_strength.push(cycle * reg);
                cycle += 1;
                reg += instruction.1;
            }
            "noop" => {
                signal_strength.push(cycle * reg);
                cycle += 1;
            }
            _ => panic!("Unknown instruction '{:?}'", instruction.0),
        }
    }

    return signal_strength;
}

fn calculate_signal_strength(instructions: &Vec<Instruction>) -> isize {
    let signal_strengths = calculate_individual_signal_strengths(&instructions);

    return signal_strengths[20]
        + signal_strengths[60]
        + signal_strengths[100]
        + signal_strengths[140]
        + signal_strengths[180]
        + signal_strengths[220];
}

fn main() {
    let instructions = parse_input("assets/input.in");

    println!("Solution 1: {:?}", calculate_signal_strength(&instructions));
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_signal_strenghts() {
        let signal_strengths =
            calculate_individual_signal_strengths(&parse_input("assets/example.in"));
        assert_eq!(420, signal_strengths[20]);
        assert_eq!(1140, signal_strengths[60]);
        assert_eq!(1800, signal_strengths[100]);
        assert_eq!(2940, signal_strengths[140]);
        assert_eq!(2880, signal_strengths[180]);
        assert_eq!(3960, signal_strengths[220]);
    }

    #[test]
    fn validate_example_1() {
        assert_eq!(
            13140,
            calculate_signal_strength(&parse_input("assets/example.in"))
        );
    }

    #[test]
    fn validate_example_2() {
        //assert_eq!(13140, calculate_individual_signal_strengths(&parse_input("assets/example.in")));
    }
}
