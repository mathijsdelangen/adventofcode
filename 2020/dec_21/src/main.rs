use std::fs;
use std::collections::HashMap;

fn intersect(first: &Vec<String>, second: &Vec<String>) -> Vec<String> {
    let mut intersection = Vec::new();
    for a in first {
        for b in second {
            if a == b { intersection.push(a.to_owned()) }
        }
    }
    return intersection;
}

fn find_solution(input_file: &str) -> u32 {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");

    let input_lines : Vec<&str> = input.split("\r\n").collect();
    let mut possible_ingredients_with_allergen : HashMap<String, Vec<String>> = HashMap::new();

    let mut all_ingredients : Vec<String> = Vec::new();

    for l in &input_lines {
        let line = l.replace(")","");
        let line_parts : Vec<String> = line.split(" (contains ")
                                        .map(|l| l.trim())
                                        .map(|l| l.to_string())
                                        .collect();
        let ingredients : Vec<String> = line_parts[0].split(" ").map(|l| l.to_string()).collect();
        let allergens : Vec<String> = line_parts[1].split(",").map(|l| l.trim()).map(|l| l.to_string()).collect();

        all_ingredients.append(ingredients.to_owned().as_mut());

        for allergen in allergens {
            // If exists, check which ingredients are the same
            if possible_ingredients_with_allergen.contains_key(&allergen) {
                let possible_ingredients_with_this_allergen = possible_ingredients_with_allergen[&allergen].to_owned();
                let intersection = intersect(&possible_ingredients_with_this_allergen, &ingredients);
                possible_ingredients_with_allergen.insert(allergen.to_string(), intersection);
            } else {
                possible_ingredients_with_allergen.insert(allergen.to_string(), ingredients.to_owned());
            }
        }
    }

    let mut possible_ingredients : Vec<String> = Vec::new();

    for allergen in possible_ingredients_with_allergen.keys() {
        println!("Allergen {} has possible ingredients:", allergen);
        for ingredient in &possible_ingredients_with_allergen[allergen] {
            println!(" {}", ingredient);
            possible_ingredients.push(ingredient.to_owned());
        }
    }

    let mut count = 0;
    for ingredient in all_ingredients {
        if !possible_ingredients.contains(&ingredient) {
            count += 1;
        }
    }

    return count;
}

fn main() {
    let input_file = "assets/dec_21.in";
    println!("Solution: {}", find_solution(input_file));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_example() {
        assert_eq!(5, find_solution("assets/dec_21_example.in"));
    }
}
