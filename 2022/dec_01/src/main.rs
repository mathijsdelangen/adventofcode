use std::fs;

fn read_input(input_file: &str) -> Vec<i32>
{
    let mut result = Vec::new();
    result.push(0);

    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let input_lines : Vec<&str> = input.split("\n").map(|l| l.trim()).collect();

    for input in input_lines {
        if input.is_empty()
        {
            result.push(0)
        }
        else {
            let val = input.parse::<i32>().unwrap();
            let last_index = result.len() - 1;
            result[last_index] = result[last_index] + val;
        }
    }

   result
}

fn first_solution(carried_food: &Vec<i32>) -> i32
{
    return *carried_food.iter().max().unwrap();
}

fn second_solution(carried_food: &Vec<i32>) -> i32
{
    let mut food = carried_food.to_vec();
    food.sort_by(|a,b| b.cmp(a));
    return food[0] + food[1] + food[2];
}

fn main() 
{
    let input_file = read_input("assets/input.in");
    println!("Solution 1: {}", first_solution(&input_file));
    println!("Solution 2: {}", second_solution(&input_file));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_sol1(){
        assert_eq!(24000, first_solution(&read_input("assets/example.in")));
    }

    #[test]
    fn validate_sol2(){
        assert_eq!(45000, second_solution(&read_input("assets/example.in")));
    }
}