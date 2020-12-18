use std::fs;
mod eval;

fn parse_input(input: &str) -> Vec<String> {
    let input_lines : Vec<String> = input.split("\r\n")
                                         .map(|l| l.chars()
                                                   .filter(|c| !c.is_whitespace())
                                                   .collect())
                                         .collect();
    return input_lines
}

fn solve_expression(exp: &str) -> u128 {
    let e = eval::Expr::from_string(exp);
    return e.get_value();
}

fn main() {
    let input_file = "assets/dec_18.in";
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let expressions = parse_input(&input);
    let result : u128 = expressions.iter().map(|x| solve_expression(x)).sum();

    println!("Solution: {}", result );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_example(){
    //    assert_eq!(26, solve_expression("2 * 3 + (4 * 5)"));
    //    assert_eq!(437, solve_expression("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
    //    assert_eq!(12240, solve_expression("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
    //    assert_eq!(13632, solve_expression("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));
    }

    #[test]
    fn validate_example2(){
        assert_eq!(51, solve_expression("1 + (2 * 3) + (4 * (5 + 6))"));
        assert_eq!(46, solve_expression("2 * 3 + (4 * 5)"));
        assert_eq!(669060, solve_expression("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
        assert_eq!(23340, solve_expression("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));
    }
}