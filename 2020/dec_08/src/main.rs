use std::fs;

// Function returns new relative ic and added acc
fn parse_instruction(line: &str) -> (i32, i32) {
    let instruction: Vec<&str> = line.split(" ").map(|l| l.trim()).collect();
    
    let instruction_value = instruction[1].replace("+", "");
    println!("'{}'", instruction_value);
    match instruction[0] {
        "acc" => return (1, instruction_value.parse::<i32>().unwrap()),
        "nop" => return (1,0), 
        "jmp" => return (instruction_value.parse::<i32>().unwrap(), 0),
        _     => panic!("Instruction unknown")
    }
}

fn find_solution (input_file: &str) -> i32 {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let input_lines : Vec<&str> = input.split("\n").map(|l| l.trim()).collect();

    let mut acc : i32 = 0;
    let mut ic : i32 = 0;

    let mut ic_visited = Vec::new();

    loop {
        ic_visited.push(ic);

        let (ic_,acc_) = parse_instruction(input_lines[ic as usize]);
        
        ic += ic_;
        if ic_visited.contains(&ic) {
            break;
        }

        acc += acc_;
    }

    return acc;
}

fn main() {
    let input_file = "assets/dec_08.in";
    println!("Solution 1: {}", find_solution(input_file));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_sol1(){
        assert_eq!(5, find_solution("assets/dec_08_example_input.in"));
    }
}