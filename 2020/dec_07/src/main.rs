mod regulations;

use std::fs;

fn get_color_from_string(line: &str) -> (u32, String){
    let mut s =  line.replace(".", "")
                     .replace("bags", "")
                     .replace("bag", "");
    s = s.trim().to_string();
    let parts : Vec<&str> = s.split(" ").map(|l| l.trim()).collect();
    match parts.len() {
        2 => return (1, s),
        3 => return (parts[0].parse::<u32>().unwrap(), format!("{} {}", parts[1], parts[2])),
        _ => panic!("Unkown number of parts"),
    }
}

fn parse_line(line: &str, mut regulations: regulations::Regulations) -> regulations::Regulations {
    let bag_info : Vec<&str> = line.split("contain").map(|l| l.trim()).collect();
    let (_, bag_color) = get_color_from_string(bag_info[0]);
    regulations.add_bag(&bag_color);
    
    let bag_contents = bag_info[1];
    match bag_contents { 
        "no other bags." => (),
        _               => 
                        {
                            let allowed_bags_info : Vec<&str> = bag_contents.split(",").map(|l| l.trim()).collect();
                            for bag in allowed_bags_info {
                                let (nr_bags, allowed_bag_color) = get_color_from_string(bag);
                                for _ in 0..nr_bags {
                                   regulations.add_bag_content(&bag_color, &allowed_bag_color);
                                }
                                
                            }
                        },
    }
    return regulations;
}

fn parse_regulations(input_file: &str) -> regulations::Regulations {

    //  Parse regulations
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let input_lines : Vec<&str> = input.split("\n")
                                        .map(|l| l.trim())
                                        .collect();

    let mut regulations = regulations::Regulations::new();
    for line in input_lines {
        regulations = parse_line(line, regulations);
    }
    return regulations;
}

fn find_solution1(input_file: &str) -> u32 {
    let regulations = parse_regulations(input_file);

    // Find solution one
    let mut count = 0;
    for bag in regulations.get_known_bags() {
        if regulations.can_contain(bag, String::from("shiny gold")) {
            count += 1;
        }
    }

    return count;
}

fn find_solution2(input_file: &str) -> usize {
    let regulations = parse_regulations(input_file);

    return regulations.contains_nr_bags(String::from("shiny gold"));
}

fn main() {
    let input = "assets/dec_07.in";
    println!("Solution 1: {}", find_solution1(input));
    println!("Solution 2: {}", find_solution2(input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_sol1(){
        assert_eq!(4, find_solution1("assets/test_input_sol1.in"));
    }

    #[test]
    fn validate_sol2(){
        assert_eq!(126, find_solution2("assets/test_input_sol2.in"));
    }
}