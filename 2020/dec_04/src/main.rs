mod constants;

fn validate_range(value: &str, min: u32, max: u32) -> bool {
    let value = value.parse::<u32>().unwrap();
    return min <= value && value <= max;
}

fn validate_prop(property: &str, property_value: &str) -> bool {
    match property {
        "byr" => return validate_range(property_value, 1920, 2002),
        "iyr" => return validate_range(property_value, 2010, 2020),
        "eyr" => return validate_range(property_value, 2020, 2030),
        "hgt" => {
            if property_value.ends_with("in") 
            {
                let height = property_value.strip_suffix("in").unwrap();
                return validate_range(height, 59, 76);
            }
            if property_value.ends_with("cm") 
            {
                let height = property_value.strip_suffix("cm").unwrap();
                return validate_range(height, 150, 193);
            }
            return false;
        }, 
        "hcl" => {
            match &property_value[..1] {
                "#" => {
                    for c in property_value[1..].chars() {
                        match c {
                            '0'..='9' | 'a'..='f' => continue,
                            _ => return false,
                        }
                    }
                    return property_value[1..].len() == 6;
                },
                _ => return false,
            }
        },
        "ecl" => {
            match property_value {
                "amb"|"blu"|"brn"|"gry"|"grn"|"hzl"|"oth" => return true,
                _ => return false,
            }
        },
        "pid" => {
            for c in property_value.chars() {
                match c {
                    '0'..='9' => continue,
                    _ => return false,
                }
            }
            return property_value.len() == 9;
        },
        "cid" => return false, // Dont care
        _     => panic!("Cannot match any property: {}", property),
    }
}

fn validate_input() -> u32 {
    let mut props : Vec<&str> = Vec::new();
    let mut result = 0;

    for line in constants::INPUT.iter() {
        match line.as_ref() {
            "" => { if props.len() == 7 {result += 1}; props.clear() },
            _ => {
                for property in line.split(" ") {
                    let match_id = &property[..3];
                    let match_value = &property[4..];
                    if validate_prop(match_id, match_value) {
                        props.push(match_id)
                    }
                }
            },
        }
    }

    return result;
}

fn main() {
    println!("Solution: {}", validate_input());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_byr(){
        assert_eq!(true, validate_prop("byr", "2002"));
        assert_eq!(false, validate_prop("byr", "2003"));
    }

    #[test]
    fn validate_hgt(){
        assert_eq!(true, validate_prop("hgt", "60in"));
        assert_eq!(true, validate_prop("hgt", "190cm"));
        assert_eq!(false, validate_prop("hgt", "190in"));
        assert_eq!(false, validate_prop("hgt", "190"));
    }

    #[test]
    fn validate_hcl(){
        assert_eq!(true, validate_prop("hcl", "#123abc"));
        assert_eq!(false, validate_prop("hcl", "#123abz"));
        assert_eq!(false, validate_prop("hcl", "123abc"));
    }

    #[test]
    fn validate_ecl(){
        assert_eq!(true, validate_prop("ecl", "brn"));
        assert_eq!(false, validate_prop("ecl", "wat"));
    }

    #[test]
    fn validate_pid(){
        assert_eq!(true, validate_prop("pid", "000000001"));
        assert_eq!(false, validate_prop("pid", "0123456789"));
    }
}