use std::fs;

#[derive(Debug, Copy, Clone)]
struct Assignment {
    min: usize,
    max: usize,
}

#[derive(Debug, Copy, Clone)]
struct SectionAssignment {
    pair: (Assignment, Assignment),
}
impl SectionAssignment {
    fn fully_contains(&self) -> bool {
        return (self.pair.0.min <= self.pair.1.min && self.pair.0.max >= self.pair.1.max)
            || (self.pair.1.min <= self.pair.0.min && self.pair.1.max >= self.pair.0.max);
    }

    fn have_overlap(&self) -> bool {
        return (self.pair.0.min..=self.pair.0.max).contains(&self.pair.1.min)
            || (self.pair.0.min..=self.pair.0.max).contains(&self.pair.1.max)
            || (self.pair.1.min..=self.pair.1.max).contains(&self.pair.0.min)
            || (self.pair.1.min..=self.pair.1.max).contains(&self.pair.0.max);
    }
}

fn create_assignment(line: &str) -> Assignment {
    let parts: Vec<usize> = line
        .split("-")
        .map(|part| part.parse::<usize>().unwrap())
        .collect();
    return Assignment {
        min: parts[0],
        max: parts[1],
    };
}

fn create_from_line(line: &str) -> SectionAssignment {
    let assignments: Vec<Assignment> = line
        .split(",")
        .map(|part| create_assignment(&part))
        .collect();
    return SectionAssignment {
        pair: (assignments[0], assignments[1]),
    };
}

fn parse_input(input_file: &str) -> Vec<SectionAssignment> {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let assignments = input
        .split("\n")
        .map(|line| create_from_line(line.trim()))
        .collect();

    return assignments;
}

fn first_solution(section_assignments: &Vec<SectionAssignment>) -> usize {
    return section_assignments
        .iter()
        .filter(|&sa| sa.fully_contains())
        .count();
}

fn second_solution(section_assignments: &Vec<SectionAssignment>) -> usize {
    return section_assignments
        .iter()
        .filter(|&sa| sa.have_overlap())
        .count();
}

fn main() {
    let input_file = parse_input("assets/input.in");
    println!("Solution 1: {}", first_solution(&input_file));
    println!("Solution 2: {}", second_solution(&input_file));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_example_1() {
        assert_eq!(2, first_solution(&parse_input("assets/example.in")));
    }

    #[test]
    fn validate_example_2() {
        assert_eq!(4, second_solution(&parse_input("assets/example.in")));
    }
}
