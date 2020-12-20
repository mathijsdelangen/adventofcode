use std::fs;
use regex::Regex;

fn parse_input(input_file: &str) -> (Vec<String>, Vec<String>) {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let input_parts : Vec<&str> = input.split("\r\n\r\n").collect();

    let rule_descriptions : Vec<&str> = input_parts[0].split("\r\n").collect();
    let mut rules = Vec::new();

    for p in rule_descriptions {
        let part : Vec<&str> = p.split(":").map(|l| l.trim()).collect();
        let idx: usize = part[0].parse::<usize>().unwrap();
        if idx >= rules.len() { rules.resize(idx+1,String::from(""))}
        rules[idx] = part[1].to_string().replace("|", ")|(");
        rules[idx] = String::from("(") + &rules[idx];
        rules[idx].push_str(")");
        rules[idx] = rules[idx].replace("\"","");
    }

    let messages = input_parts[1].split("\r\n")
                                .map(|l| l.to_string())
                                .collect();

    return (rules, messages);
}

fn determine_matches((rules, messages): (Vec<String>, Vec<String>)) -> usize {
    let mut rules_filled_in = rules.to_owned();
    for i in (0..rules_filled_in.len()).rev() {
        // Special case, extend rule 11 more than once, after that remove 11 from all rules
        if i == 11 {
            for _ in 0..3 {
                for j in 0..rules.len() {
                    rules_filled_in[j] = rules_filled_in[j].replace(&i.to_string(), &(String::from("(") + &rules_filled_in[i] + ")"));
                }        
            }
            for j in 0..rules.len() {
                rules_filled_in[j] = rules_filled_in[j].replace(&i.to_string(), "");
            }        
        }
        for j in 0..rules.len() {
            rules_filled_in[j] = rules_filled_in[j].replace(&i.to_string(), &(String::from("(") + &rules_filled_in[i] + ")"));
        }
    }

    let mut result = 0;
    rules_filled_in[0] = rules_filled_in[0].replace(" ","");
    rules_filled_in[0] = String::from("^") + &rules_filled_in[0];
    rules_filled_in[0].push_str("$");
//    println!("Complete regex: {}", rules_filled_in[0]);
    let re = Regex::new(&rules_filled_in[0]).unwrap();
    for m in messages {
        let mat = re.find(&m);
        match mat {
            Some(value) => if value.start() == 0 && value.end() == m.len() { result += 1}
            _ => (),
        }
    }

    return result;
}

fn main() {
    let input_file = "assets/dec_19.in";

    println!("Solution: {}", determine_matches(parse_input(input_file)) );
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_example(){
        let input_file = "assets/dec_19_example.in";
        assert_eq!(2, determine_matches(parse_input(input_file)));
    }

    #[test]
    fn validate_example2a(){
        let input_file = "assets/dec_19_example2a.in";
        assert_eq!(3, determine_matches(parse_input(input_file)));
    }

    #[test]
    fn validate_example2b(){
        let input_file = "assets/dec_19_example2b.in";
        assert_eq!(12, determine_matches(parse_input(input_file)));
    }
}