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

fn difference(first: &Vec<String>, second: &Vec<String>) -> Vec<String> {
    let mut new_list = Vec::new();
    for a in first {
        if !second.contains(a) { new_list.push(a.to_owned()) }
    }
    return new_list;
}

fn find_solution(input_file: &str) -> Vec<String> {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let input_lines : Vec<&str> = input.split("\r\n").collect();

    let mut possible_ingredients_with_allergen : HashMap<String, Vec<String>> = HashMap::new();

    for l in &input_lines {
        let line = l.replace(")","");
        let line_parts : Vec<String> = line.split(" (contains ")
                                        .map(|l| l.trim())
                                        .map(|l| l.to_string())
                                        .collect();
        let ingredients : Vec<String> = line_parts[0].split(" ")
                                                     .map(|l| l.to_string())
                                                     .collect();
        let allergens : Vec<String> = line_parts[1].split(",")
                                                   .map(|l| l.trim())
                                                   .map(|l| l.to_string())
                                                   .collect();

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

    for allergen in possible_ingredients_with_allergen.keys() {
        println!("Allergen {} has possible ingredients:", allergen);
        for ingredient in &possible_ingredients_with_allergen[allergen] {
            println!(" {}", ingredient);
        }
    }

    let mut dangerous_ingredients : Vec<String> = Vec::new();
    let mut ingredients_with_allergen : HashMap<String, Vec<String>> = HashMap::new();
    let mut all_known = false;
    while !all_known {
        all_known = true;
        for (allergen, ingredients) in &possible_ingredients_with_allergen {
            if difference(&ingredients, &dangerous_ingredients).len() == 1 {
                all_known = false;
                let differences = difference(&ingredients, &dangerous_ingredients);
                ingredients_with_allergen.insert(allergen.to_owned(),differences.to_owned());
                dangerous_ingredients.push(differences[0].to_owned());
            }
        }
    }
   
    // Sort hashmap to vector
    let mut allergens : Vec<_> = ingredients_with_allergen.into_iter().collect();
    allergens.sort_by(|x,y| x.0.cmp(&y.0));
    return allergens.iter().map(|a| a.1[0].to_owned()).collect();
}

fn main() {
    let input_file = "assets/dec_21.in";
    println!("Solution: {}", find_solution(input_file).join(","));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_example() {
        assert_eq!("mxmxvkd,sqjhc,fvjkl", find_solution("assets/dec_21_example.in").join(","));
    }
}
