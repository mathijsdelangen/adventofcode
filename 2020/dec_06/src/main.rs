use std::fs;

fn count_yes_answers(answers: &String) -> usize {
 
    let mut answers_vec: Vec<char> = answers.chars().collect();
    answers_vec.sort_by(|a,b| b.cmp(a));
    answers_vec.dedup();

    return answers_vec.len();
}

fn count_all_yes_answers(answers: &String, people_in_group: usize) -> usize {

    let answers_vec: Vec<char> = answers.chars().collect();

    let mut answers_given = answers_vec.clone();
    answers_given.sort_by(|a,b| b.cmp(a));
    answers_given.dedup();

    let mut result = 0;
    for answer in answers_given {
        if answers_vec.iter().filter(|&a| *a == answer).count() == people_in_group {
            result += 1;
        }
    }

    return result;
}

fn main() {
    let input = fs::read_to_string("assets/dec_06.in").expect("Something went wrong reading the file");
    let input_lines : Vec<&str> = input.split("\n")
                                       .map(|l| l.trim())
                                       .collect();
    
    let mut result_sol_1 = 0;
    let mut result_sol_2 = 0;
    let mut answers: String = String::from("");
    let mut people_in_group: usize = 0;

    for line in input_lines {
        if line.trim().is_empty() { 
            result_sol_1 += count_yes_answers(&answers); 
            result_sol_2 += count_all_yes_answers(&answers, people_in_group); 
            answers.clear(); 
            people_in_group = 0;
        }
        else {
            answers.push_str(line); people_in_group += 1;
        }
    }

    println!("Solution 1: {}", result_sol_1);
    println!("Solution 2: {}", result_sol_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_count_yes_answers(){
        assert_eq!(3, count_yes_answers(&String::from("abcabc")));
        assert_eq!(3, count_yes_answers(&String::from("abc")));
        assert_eq!(3, count_yes_answers(&String::from("abac")));
        assert_eq!(1, count_yes_answers(&String::from("aaaa")));
        assert_eq!(1, count_yes_answers(&String::from("b")));
    }

    #[test]
    fn validate_count_all_yes_answers(){
        assert_eq!(3, count_all_yes_answers(&String::from("abcabc"), 2));
        assert_eq!(0, count_all_yes_answers(&String::from("abc"), 3));
        assert_eq!(1, count_all_yes_answers(&String::from("abac"), 2));
        assert_eq!(1, count_all_yes_answers(&String::from("aaaa"), 4));
        assert_eq!(1, count_all_yes_answers(&String::from("b"), 1));
    }
}