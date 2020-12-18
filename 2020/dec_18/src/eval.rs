enum Operator {
    None,
    Add,
    Mult,
    Par,
}

pub struct Expr {
    op: Operator,
    expr_l: Option<Box<Expr>>,
    expr_r: Option<Box<Expr>>,
    exp: String,
}

impl Expr {
    fn new() -> Expr {
        return Expr {op:Operator::None, expr_l:None, expr_r:None, exp: String::from("")}
    }

    pub fn get_value (&self) -> u128 {
        if self.expr_l.is_none() && self.expr_r.is_none() { return self.exp.parse::<u128>().unwrap() }
        //println!("left: {}, right: {}", self.expr_l.as_ref().unwrap().exp, self.expr_r.as_ref().unwrap().exp);
        match self.op {
            Operator::None => panic!("This statement should never be reached"),
            Operator::Par =>  return self.expr_l.as_ref().unwrap().get_value(),
            Operator::Add  => return self.expr_l.as_ref().unwrap().get_value() + self.expr_r.as_ref().unwrap().get_value(),
            Operator::Mult => return self.expr_l.as_ref().unwrap().get_value() * self.expr_r.as_ref().unwrap().get_value(),
        }
    }

    fn find_operator_outside_parentheses(operator: char, exp: &str) -> Option<usize> {
        let mut inside_parentheses_lvl = 0;
        for (i,c) in exp.char_indices() {
            match c {
                '(' => inside_parentheses_lvl += 1,
                ')' => inside_parentheses_lvl -= 1,
                _ => if c == operator && inside_parentheses_lvl == 0 {return Some(i)},
            }
        }
        return None;
    }

    pub fn from_string(exp: &str) -> Expr {
        //println!("exp '{}'", exp);

        // Find multiplications (lowest precesence level first) 
        let idx = Expr::find_operator_outside_parentheses('*', exp);
        match idx {
            Some(i) => {
                let expr_l = Expr::from_string(&exp[..i].trim());
                let expr_r = Expr::from_string(&exp[i+1..].trim());
                //println!("Found * with: {} and {}", expr_l.exp, expr_r.exp);
                return Expr {op: Operator::Mult, expr_l:Some(Box::new(expr_l)) ,expr_r:Some(Box::new(expr_r)), exp: exp.to_string()}
            }
            None => (),
        }

        // Find additions
        let idx = Expr::find_operator_outside_parentheses('+', exp);
        match idx {
            Some(i) => {
                let expr_l = Expr::from_string(&exp[..i].trim());
                let expr_r = Expr::from_string(&exp[i+1..].trim());
                //println!("Found + with: {} and {}", expr_l.exp, expr_r.exp);
                return Expr {op: Operator::Add, expr_l:Some(Box::new(expr_l)) ,expr_r:Some(Box::new(expr_r)), exp: exp.to_string()}
            }
            None => (),
        }

        // Check if we are dealing with values inside parentheses
        if exp.chars().nth(0).unwrap() == '(' && exp.chars().nth(exp.len()-1).unwrap() == ')' {
            let expr_l = Expr::from_string(&exp[1..exp.len()-1].trim());
            return Expr {op: Operator::Par, expr_l:Some(Box::new(expr_l)) ,expr_r:None, exp: exp[1..exp.len()-1].to_string()};
        }

        return Expr {op: Operator::None, expr_l:None, expr_r:None, exp: exp.to_string()};
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_get_value(){
        assert_eq!(2, Expr {op: Operator::None, expr_l:None, expr_r:None, exp: String::from("2")}.get_value());
        assert_eq!(2, Expr::from_string("2").get_value());
        assert_eq!(3, Expr::from_string("1 + 2").get_value());
        assert_eq!(6, Expr::from_string("3 * 2").get_value());
        assert_eq!(6, Expr::from_string("1 * 2 * 3").get_value());
        assert_eq!(14, Expr::from_string("2 * 3 + 4").get_value());
        assert_eq!(14, Expr::from_string("4 + 3 * 2").get_value());
    }

    #[test]
    fn validate_get_value_with_parentheses(){
        assert_eq!(2, Expr {op: Operator::None, expr_l:None, expr_r:None, exp: String::from("2")}.get_value());
        assert_eq!(2, Expr::from_string("2").get_value());
        assert_eq!(3, Expr::from_string("(1 + 2)").get_value());
        assert_eq!(6, Expr::from_string("(3 * 2)").get_value());
        assert_eq!(6, Expr::from_string("(1 * 2) * 3").get_value());
        assert_eq!(6, Expr::from_string("1 * (2 * 3)").get_value());
        assert_eq!(10, Expr::from_string("(2 * 3) + 4").get_value()); // this one is different from above
        assert_eq!(10, Expr::from_string("4 + (3 * 2)").get_value()); // this one is different from above
    }

}