use std::collections::HashMap;
use std::fs;

type AllowedRanges = Vec<std::ops::RangeInclusive<u32>>;
type TicketRules = HashMap<String, AllowedRanges>;
type Ticket = Vec<u32>;
type TicketInput = (TicketRules, Ticket, Vec<Ticket>);

fn parse_ticket_rule(rule_line: &str) -> (String, AllowedRanges) {
    let rule_info : Vec<&str> = rule_line.split(": ").collect();
    let rule_name = rule_info[0];
    let rule_bounds : Ticket = rule_info[1]
                        .replace(" or ",",")
                        .replace("-",",")
                        .split(",")
                        .map(|v| v.parse::<u32>().unwrap())
                        .collect();

    return (String::from(rule_name), vec![rule_bounds[0]..=rule_bounds[1], rule_bounds[2]..=rule_bounds[3]]); 
}

fn parse_ticket(ticket_line: &str) -> Ticket {
    let ticket : Ticket = ticket_line
                                .split(",")
                                .map(|v| v.trim())
                                .map(|v| v.parse::<u32>().unwrap())
                                .collect();
    return ticket;
}

fn parse_input(input_file: &str) -> TicketInput {
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

fn is_valid_in_ranges(value: u32, ranges: &AllowedRanges) -> bool {  

    return ranges.iter().any(|r| r.contains(&value));
}

fn get_all_valid_ranges(rules: &TicketRules) -> AllowedRanges {
    let mut ranges = Vec::new();
    for (_, valid_ranges) in rules {
        for range in valid_ranges {
            ranges.push(*range.start()..=*range.end());
        }
    }
    return ranges;
}

fn ticket_scanning_error_rate((ticket_rules, _my_ticket, nearby_tickets): &TicketInput) -> u32 {
    let mut invalid_values = Vec::new();
    for ticket in nearby_tickets {
        for ticket_value in ticket {
            if !is_valid_in_ranges(*ticket_value, &get_all_valid_ranges(&ticket_rules)) { invalid_values.push(*ticket_value) }
        }
    }

    return invalid_values.iter().sum();
}

fn get_valid_tickets((ticket_rules, _my_ticket, nearby_tickets): &TicketInput) -> Vec<&Ticket> {
    let valid_tickets = nearby_tickets.iter()
                                      .filter(|t| t.iter()
                                                   .all(|&v| is_valid_in_ranges(v, &get_all_valid_ranges(&ticket_rules))))
                                      .collect();
    return valid_tickets;
}

fn determine_fields(ticket_rules: &TicketRules, nearby_tickets: Vec<&Ticket>) -> Vec<String> {
    let mut remaining_fields : Vec<String> = ticket_rules.keys().cloned().collect();
    let mut fields = vec![String::from(""); nearby_tickets[0].len()];

    while remaining_fields.len() > 0 {
        println!("{} remaining fields left", remaining_fields.len());
        for field_location in 0..nearby_tickets[0].len() {
            if fields[field_location] != "" { continue; }
            println!("Determine field location {}", field_location);
            let mut nr_possibilities = 0;
            let mut last_field_found = "";
            for field in &remaining_fields {
                if nearby_tickets.iter().all(|t| is_valid_in_ranges(t[field_location], ticket_rules.get(field).unwrap())) {
                    nr_possibilities += 1;
                    last_field_found = field;
                }
            }
            if nr_possibilities == 1 {
                println!("Store '{}' at location {}", last_field_found, field_location);
                fields[field_location] = String::from(last_field_found);
                let index = remaining_fields.iter().position(|x| *x == fields[field_location]).unwrap();
                remaining_fields.remove(index);
            }
        }
    }
        
    return fields;
}

fn main() {
    let input = "assets/dec_16.in";
    let info = parse_input(input);
    println!("Solution 1: {}", ticket_scanning_error_rate(&info));

    let valid_tickets = get_valid_tickets(&info);
    let fields = determine_fields(&info.0, valid_tickets);
    let mut result : u128 = 1;

    for i in 0..fields.len() {
        if fields[i].starts_with("departure") {
            println!("Field@{} = {}", i, fields[i]);
            result *= info.1[i] as u128;
        }
    }

    println!("Solution 2: {}", result)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_example() {
        assert_eq!(71, ticket_scanning_error_rate(&parse_input("assets/dec_16_example.in")));
    }
}
