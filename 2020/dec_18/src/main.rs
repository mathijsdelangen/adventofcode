use std::fs;

enum Op {
    None,
    Add,
    Mult,
}

fn parse_input(input: &str) -> Vec<String> {
    let input_lines : Vec<String> = input.split("\r\n")
                                         .map(|l| l.chars()
                                                   .filter(|c| !c.is_whitespace())
                                                   .collect())
                                         .collect();
    return input_lines
}

fn eval_value(val: u128, op: &Op, current_value: u128) -> u128 {
    match op {
        Op::None => return val,
        Op::Add => return current_value + val,
        Op::Mult => return current_value * val,
    }
}

fn parse_expression(exp: &str) -> (u128,usize) {
    let mut current_op = Op::None;
    let mut result = 0;
    let mut index = 0;
    loop {
        let c = exp.chars().nth(index);
        match c {
            Some(' ') => (),
            Some('+') => current_op = Op::Add,
            Some('*') => current_op = Op::Mult,
            Some('(') => {
                let (par_result,idx) = parse_expression(&exp[index+1..]);
                result = eval_value(par_result, &current_op, result);
                index += idx;
            },
            Some(')') => return (result,index+1),
            Some(e)   => {
                let val = &e.to_string().parse::<u128>().unwrap();
                result = eval_value(*val, &current_op, result)
            }
            None      => break
        }
        index += 1;
    }
  
    return (result, index);
}

fn solve_expression(exp: &str) -> u128 {
    println!("Solve '{}'", exp);
    return parse_expression(exp).0;
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
        assert_eq!(26, solve_expression("2 * 3 + (4 * 5)"));
        assert_eq!(437, solve_expression("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(12240, solve_expression("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
        assert_eq!(13632, solve_expression("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));
    }
}