pub struct Password {
    min: usize,
    max: usize,
    c: String,
    pw: String,
}

impl Password {
    pub fn new(min:usize, max: usize, c: &str, pw: &str) -> Password {
        Password {
        min: min,
        max: max,
        c: String::from(c),
        pw: String::from(pw),
        }
    }

    pub fn valid_policy1(&self) -> bool {
        let count = &self.pw.matches(&self.c).count();
        return &self.min <= count && count <= &self.max;
    }

    pub fn valid_policy2(&self) -> bool {
        let character = &self.c.chars().nth(0).unwrap();
        
        let mut min_char_valid = false;
        if self.min <= self.pw.chars().count() {
           let min_char = &self.pw.chars().nth(self.min-1).unwrap();
           min_char_valid = min_char == character;
        }
        let mut max_char_valid = false;
        if self.max <= self.pw.chars().count() {
            let max_char = &self.pw.chars().nth(self.max-1).unwrap();
            max_char_valid = max_char == character;
        }

        return (min_char_valid) ^ (max_char_valid);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_policy1_example_one(){
        let pw = Password::new(1, 3, "a", "abcde");
        assert_eq!(true, pw.valid_policy1());
    }

    #[test]
    fn valid_policy1_example_two(){
        let pw = Password::new(1, 3, "b", "cdefg");
        assert_eq!(false, pw.valid_policy1());
    }

    #[test]
    fn valid_policy1_example_three(){
        let pw = Password::new(2, 9, "c", "ccccccccc");
        assert_eq!(true, pw.valid_policy1());
    }

    #[test]
    fn valid_policy2_example_one(){
        let pw = Password::new(1, 3, "a", "abcde");
        assert_eq!(true, pw.valid_policy2());
    }

    #[test]
    fn valid_policy2_example_two(){
        let pw = Password::new(1, 3, "b", "cdefg");
        assert_eq!(false, pw.valid_policy2());
    }

    #[test]
    fn valid_policy2_example_three(){
        let pw = Password::new(2, 9, "c", "ccccccccc");
        assert_eq!(false, pw.valid_policy2());
    }

    #[test]
    fn valid_policy2_one_index_out_of_bounds(){
        let pw = Password::new(2, 20, "c", "ccccccccc");
        assert_eq!(true, pw.valid_policy2());
    }

    #[test]
    fn valid_policy2_both_index_out_of_bounds(){
        let pw = Password::new(25, 20, "c", "ccccccccc");
        assert_eq!(false, pw.valid_policy2());
    }

    #[test]
    fn valid_policy2_index_at_bound(){
        let pw = Password::new(1, 3, "c", "acc");
        assert_eq!(true, pw.valid_policy2());
    }
    #[test]
    fn valid_policy2_index_at_bound_plus_one(){
        let pw = Password::new(1, 4, "c", "acc");
        assert_eq!(false, pw.valid_policy2());
    }
  }