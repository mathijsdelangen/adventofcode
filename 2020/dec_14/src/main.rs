use std::fs;
use std::collections::HashMap;

fn bit_to_and_mask(bit: u32) -> u64 {
    return 2u64.pow(36)-1 - 2u64.pow(35-bit);
}

fn bit_to_or_mask(bit: u32) -> u64 {
    return 2u64.pow(35-bit);
}

fn determine_mask(mask: &str) -> (u64,u64,Vec<u64>) {
    let mut or_mask = 0;
    let and_mask = 2u64.pow(36)-1;
    let mut floating_bits = Vec::new();
    
    for c in mask.trim_start_matches("mask = ").char_indices() {
        match c.1 {
            '1' => or_mask |= bit_to_or_mask(c.0 as u32),
            '0' => (), //and_mask &= bit_to_and_mask(c.0 as u32),
            'X' => floating_bits.push(c.0 as u64),
            _  => panic!("Unknown bitfield"),
        }
    }

    return (or_mask, and_mask, floating_bits);
}

fn apply_mask(value: u64, mask: &(u64,u64, Vec<u64>)) ->  Vec<u64> {
    let floating_bits = &mask.2;
    let base_value = (value | mask.0 ) & mask.1;

    let mut values : Vec<u64> = vec![];
    // Determine masks
    for b in floating_bits {
        let or_mask = bit_to_or_mask(*b as u32);
        let and_mask = bit_to_and_mask(*b as u32);
        if values.len() == 0 {
            values.push(base_value & and_mask);
            values.push(base_value | or_mask);
        }
        for value in values.to_owned() {
            values.push(value & and_mask);
            values.push(value | or_mask);
        }
    }

    return values;
}

fn determine_memory_value(mem: &str) -> (u64,u64) {
    let memory_items : Vec<&str> = mem.split(" = ").collect();
    let mex_index = memory_items[0].trim_start_matches("mem[").trim_end_matches("]").parse::<u64>().unwrap();
    let mem_value = memory_items[1].parse::<u64>().unwrap();
    return (mex_index, mem_value);
}

fn initialize_memory(input_file: &str) -> u64 {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let input_lines : Vec<&str> = input.split("\n").map(|v| v.trim()).collect();

    let mut mask = (0,0,vec![]);
    let mut memory = HashMap::new();

    for line in input_lines {
        match &line[0..3] {
            "mas" => mask = determine_mask(line),
            "mem" => {
                let (mem_address, value) = determine_memory_value(line);
                let mem_addresses = apply_mask(mem_address, &mask);
                for mem_address in mem_addresses {
                   memory.insert(mem_address, value);
                }
            }
            _     => panic!("line cannot be identified"),
        }
    }

    return memory.iter().map(|v| v.1).sum();
}

fn main() {
    let input_file = "assets/dec_14.in";
    println!("Solution: {}", initialize_memory(input_file));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_example() {
        assert_eq!(208, initialize_memory("assets/dec_14_example2.in"));
    }
}