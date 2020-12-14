use std::fs;
use std::collections::HashMap;

fn determine_mask(mask: &str) -> (u64,u64) {
    let mut or_mask = 0;
    let mut and_mask = 2u64.pow(36)-1;
    
    for c in mask.trim_start_matches("mask = ").char_indices() {
        match c.1 {
            '1' => or_mask |= 2u64.pow(35-c.0 as u32),
            '0' => and_mask &= (2u64.pow(36)-1) - (2u64.pow(35-c.0 as u32)),
            _  => (),
        }
    }

    return (or_mask, and_mask);
}

fn apply_mask(value: u64, mask: (u64,u64)) -> u64 {
    return (value | mask.0 ) & mask.1;
}

fn determine_memory_value(mem: &str, mask: (u64,u64)) -> (u64, u64) {
    let memory_items : Vec<&str> = mem.split(" = ").collect();
    let mex_index = memory_items[0].trim_start_matches("mem[").trim_end_matches("]").parse::<u64>().unwrap();
    let mem_value = apply_mask(memory_items[1].parse::<u64>().unwrap(), mask);
    return (mex_index, mem_value);
}

fn initialize_memory(input_file: &str) -> u64 {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let input_lines : Vec<&str> = input.split("\n").map(|v| v.trim()).collect();

    let mut mask = (0,0);
    let mut memory = HashMap::new();

    for line in input_lines {
        match &line[0..3] {
            "mas" => mask = determine_mask(line),
            "mem" => {
                let mem = determine_memory_value(line, mask);
                memory.insert(mem.0, mem.1);
            }
            _     => panic!("line cannot be identified"),
        }
    }

    return memory.iter().map(|v| v.1).sum();
}

fn main() {
    let input_file = "assets/dec_14.in";
    println!("Solution 1: {}", initialize_memory(input_file));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_example() {
        assert_eq!(165, initialize_memory("assets/dec_14_example.in"));
    }
    #[test]
    fn validate_determine_mask() {
        assert_eq!((2u64.pow(6),((2u64.pow(36)-1)-(2))), determine_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"));
    }
}