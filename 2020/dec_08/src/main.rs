use std::fs;

// Function returns new relative instruction counter (ic) and accumulator (acc)
fn parse_instruction(line: &str) -> (i32, i32) {
    let instruction: Vec<&str> = line.split(" ").map(|l| l.trim()).collect();
    match instruction[0] {
        "acc" => return (1, instruction[1].parse::<i32>().unwrap()),
        "nop" => return (1,0), 
        "jmp" => return (instruction[1].parse::<i32>().unwrap(), 0),
        _     => panic!("Instruction unknown")
    }
}

// Returns value of ic and acc when loop is detected or program ends
fn run_program(input_lines: Vec<&str>) -> (i32, i32) {
    let mut acc : i32 = 0;
    let mut ic : i32 = 0;
    let mut ic_visited = Vec::new();

    loop {
        ic_visited.push(ic);
        let (ic_,acc_) = parse_instruction(input_lines[ic as usize]);
        
        ic += ic_;
        if ic_visited.contains(&ic) { println!("Loop detected"); break; }
        
        acc += acc_;
        if ic >= input_lines.len() as i32 { println!("End of program detected"); break; }
    }

    return (ic, acc);
}

fn change_instruction(line: &str) -> String {
    let instruction: Vec<&str> = line.split(" ").map(|l| l.trim()).collect();
    match instruction[0] {
        "acc" => return String::from(line),
        "nop" => return line.replace("nop", "jmp"),
        "jmp" => return line.replace("jmp", "nop"),
        _     => panic!("Instruction unknown")
    }
}

fn find_solution1(input_file: &str) -> i32 {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let input_lines : Vec<&str> = input.split("\n").map(|l| l.trim()).collect();

    let (_,acc) = run_program(input_lines);
    return acc;
}

fn find_solution2(input_file: &str) -> i32 {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let input_lines : Vec<&str> = input.split("\n").map(|l| l.trim()).collect();

    let mut ic_to_change = 0;
    loop {
        let mut test_input = input_lines.clone();
        let instruction = change_instruction(test_input[ic_to_change]);
        test_input[ic_to_change] = &instruction;
        
        let (ic, acc) = run_program(test_input);
        if ic >= input_lines.len() as i32 { return acc; }
        
        ic_to_change += 1;
    }
}

fn main() {
    let input_file = "assets/dec_08.in";
    println!("Solution 1: {}", find_solution1(input_file));
    println!("Solution 2: {}", find_solution2(input_file));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_sol1(){
        assert_eq!(5, find_solution1("assets/dec_08_example_input.in"));
    }

    #[test]
    fn validate_sol2(){
        assert_eq!(8, find_solution2("assets/dec_08_example_input.in"));
    }
}