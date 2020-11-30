fn solution(input: &str, inc : usize) -> () {
  const RADIX: u32 = 10;

  let digits : Vec<u32> = input.chars()
    .map(|c| c.to_digit(RADIX).unwrap())
    .collect();

  let mut result = 0;
  for i in 0..digits.len() {
    if digits[i] == digits[(i+inc) % digits.len()] {
      result += digits[i];
    }
  }

  println!("{}", result);
}

fn main() {
  let input = "1212";
  //solution(input, 1);
  solution(input, input.len());
}
