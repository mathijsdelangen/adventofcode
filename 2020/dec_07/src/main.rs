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
        "no other bags" => (),
        _               => 
                        {
                            let allowed_bags_info : Vec<&str> = bag_contents.split(",").map(|l| l.trim()).collect();
                            for bag in allowed_bags_info {
                                let (_, allowed_bag_color) = get_color_from_string(bag);
                                regulations.add_bag_content(&bag_color, &allowed_bag_color);
                                
                            }
                        },
    }
    return regulations;
}

fn main() {
    //  Parse regulations
    let input = fs::read_to_string("assets/dec_07.in").expect("Something went wrong reading the file");
    let input_lines : Vec<&str> = input.split("\n")
                                       .map(|l| l.trim())
                                       .collect();

    let mut regulations = regulations::Regulations::new();
    for line in input_lines {
        regulations = parse_line(line, regulations);
    }
    
    // Find solution one
    let mut count = 0;
    for bag in regulations.get_known_bags() {
        if regulations.can_contain(bag, String::from("shiny gold")) {
            count += 1;
        }
    }

    println!("Solution 1: {}", count);
}

