use std::collections::HashMap;
use std::fs;

fn parse_ticket_rule(rule_line: &str) -> (String, Vec<std::ops::RangeInclusive<u32>>) {
    let rule_info : Vec<&str> = rule_line.split(": ").collect();
    let rule_name = rule_info[0];
    let rule_bounds :Vec<u32> = rule_info[1]
                        .replace(" or ",",")
                        .replace("-",",")
                        .split(",")
                        .map(|v| v.parse::<u32>().unwrap())
                        .collect();

    return (String::from(rule_name), vec![rule_bounds[0]..=rule_bounds[1], rule_bounds[2]..=rule_bounds[3]]); 
}

fn parse_ticket(ticket_line: &str) -> Vec<u32> {
    let ticket : Vec<u32> = ticket_line
                                .split(",")
                                .map(|v| v.trim())
                                .map(|v| v.parse::<u32>().unwrap())
                                .collect();
    return ticket;
}

fn parse_input(input_file: &str) -> (HashMap<String, Vec<std::ops::RangeInclusive<u32>>>, Vec<u32>, Vec<Vec<u32>>) {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let input_parts : Vec<&str> = input.split("\r\n\r\n").map(|v| v.trim()).collect();
    let input_rules : Vec<&str> = input_parts[0].split("\r\n").map(|v| v.trim()).collect();
    let input_my_ticket = input_parts[1];
    let input_nearby_tickets : Vec<&str> = input_parts[2].split("\r\n").map(|v| v.trim()).collect();

    let ticket_rules = input_rules.iter().map(|r| parse_ticket_rule(r)).collect();
    let my_ticket = parse_ticket(input_my_ticket);
    let nearby_tickets = input_nearby_tickets.iter().map(|t| parse_ticket(t)).collect();

    return (ticket_rules, my_ticket, nearby_tickets);
}

fn get_all_valid_ranges(rules: &HashMap<String, Vec<std::ops::RangeInclusive<u32>>>) -> Vec<&std::ops::RangeInclusive<u32>>{
    let mut ranges = Vec::new();
    for (_, valid_ranges) in rules {
        for range in valid_ranges {
            ranges.push(range);
        }
    }
    return ranges;
}

fn ticket_scanning_error_rate((ticket_rules, _my_ticket, nearby_tickets): (HashMap<String, Vec<std::ops::RangeInclusive<u32>>>, Vec<u32>, Vec<Vec<u32>>)) -> u32 {
    let mut invalid_values = Vec::new();
    for ticket in nearby_tickets {
        for ticket_value in ticket {
            let mut has_valid_range = false;
            for range in get_all_valid_ranges(&ticket_rules) {
                if range.contains(&ticket_value) { has_valid_range = true }
            }
            if !has_valid_range { invalid_values.push(ticket_value) }
        }
    }

    return invalid_values.iter().sum();
}

fn main() {
    let input = "assets/dec_16.in";

    println!("Solution: {}", ticket_scanning_error_rate(parse_input(input)))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_example() {
        assert_eq!(71, ticket_scanning_error_rate(parse_input("assets/dec_16_example.in")));
    }
}
