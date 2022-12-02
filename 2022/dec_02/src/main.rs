use std::fs;

struct Round {
    opponent_choice: char,
    player_choice: char,
}

fn calculate_round_score(opponent_choice: char, player_choice: char) -> i32
{
    match opponent_choice{
        'A' => 
        {
            match player_choice{
                'X' => return 0 + 3,
                'Y' => return 3 + 1, //OK
                'Z' => return 6 + 2,
                _ => panic!("unknown player input {}", player_choice),
            };
        }
        'B' => 
        {
            match player_choice{
                'X' => return 0 + 1, // OK
                'Y' => return 3 + 2,
                'Z' => return 6 + 3,
                _ => panic!("unknown player input {}", player_choice),
            };
        }
        'C' => 
        {
            match player_choice{
                'X' => return 0 + 2,
                'Y' => return 3 + 3,
                'Z' => return 6 + 1, // OK
                _ => panic!("unknown player input {}", player_choice),
            };
        }
        _ => panic!("unknown oppenent input {}", opponent_choice),
    };
}

fn calculcate_score(rounds: Vec<Round>) -> i32
{
    let mut total_score = 0;
    for round in rounds.iter(){
        total_score += calculate_round_score(round.opponent_choice, round.player_choice);
    }

    return total_score;
}

fn read_input(input_file: &str) -> Vec<Round>
{
    let mut rounds = Vec::new();

    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let input_lines: Vec<&str> = input.split("\n").map(|l| l.trim()).collect();
    for input_line in input_lines.iter()
    {
        let chars: Vec<char> = input_line.chars().collect();
        rounds.push(Round { opponent_choice: chars[0], player_choice: chars[2] });
    }
    
    return rounds;
}

fn _first_solution(inputs: Vec<Round>) -> i32
{
    return calculcate_score(inputs);
}
 
fn second_solution(inputs: Vec<Round>) -> i32
{
    return calculcate_score(inputs);
}

fn main() 
{
    let input_file = read_input("assets/input.in");
    //println!("Solution 1: {}", first_solution(input_file));
    println!("Solution 2: {}", second_solution(input_file));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_sol1(){
        //assert_eq!(15, first_solution(read_input("assets/example.in")));
    }

    #[test]
    fn validate_sol2(){
        assert_eq!(12, second_solution(read_input("assets/example.in")));
    }
}