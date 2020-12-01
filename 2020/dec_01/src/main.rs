mod constants;

fn find_sum(number_of_values: u32, sum: i32) -> i32 {
  let mut result = 0;
  for value in constants::INPUT.iter(){
    let find_value = sum - value;
    if number_of_values > 1 {
      let sum2 = find_sum(number_of_values-1, find_value);
      if sum2 > 0 {
        result = value*sum2;
      }
      else {
        if constants::INPUT.contains(&find_value){
          result = value * find_value;
        }
      }
    }
  }
  return result;
}

fn main() {
  print!("{}",find_sum(2, 2020));
}
