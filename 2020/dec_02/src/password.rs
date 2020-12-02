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

    fn char_at_location_in_password(&self, char_to_check: &char, location: usize) -> bool {
        if location >= self.pw.chars().count() { return false }

        let char_in_string = self.pw.chars().nth(location);
        match char_in_string {
            Some(c) => return c == *char_to_check,
            _       => return false
        }
    }
    pub fn valid_policy2(&self) -> bool {
        let character = &self.c.chars().nth(0).unwrap();
        
        let min_char_valid = self.char_at_location_in_password(character, self.min-1);
        let max_char_valid = self.char_at_location_in_password(character, self.max-1);

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