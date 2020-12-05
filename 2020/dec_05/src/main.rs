mod constants;

use std::cmp;

fn get_row(rows: &str) -> u32 {
    let mut min = 0;
    let mut max = 127;

    for row in rows.chars() {
        match row {
            'F' => max -= (max+1-min) / 2,
            'B' => min += (max+1-min) / 2,
            _   => panic!("Invalid input"),
        }
    }
    return min;
}

fn get_column(columns: &str) -> u32 {
    let mut min = 0;
    let mut max = 7;

    for column in columns.chars() {
        match column {
            'L' => max -= (max+1-min) / 2,
            'R' => min += (max+1-min) / 2,
            _   => panic!("Invalid input"),
        }
    }
    return min;
}

fn get_seat_id(boarding_pass: &str) -> u32 {

    return get_row(&boarding_pass[..7]) * 8 + get_column(&boarding_pass[7..]);
}

fn main() {
    let mut result = 0;
    for line in constants::INPUT.iter() {
        result = cmp::max(result, get_seat_id(line));
    }

    println!("Solution 1: {}", result);

    let mut inputs : Vec<u32> = constants::INPUT
        .to_vec()
        .iter()
        .map(|i| get_seat_id(i))
        .collect();
    inputs.sort();
    
    for i in 0..inputs.len()-1 {
        if inputs[i] != inputs[i+1]-1 {
            println!("Solution 2: {} (gap between {} and {})", inputs[i]+1, inputs[i], inputs[i+1]);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_get_row(){
        assert_eq!(44, get_row("FBFBBFF"));
    }

    #[test]
    fn validate_get_column(){
        assert_eq!(5, get_column("RLR"));
    }

    #[test]
    fn validate_get_seat_id(){
        assert_eq!(567, get_seat_id("BFFFBBFRRR"));
        assert_eq!(119, get_seat_id("FFFBBBFRRR"));
        assert_eq!(820, get_seat_id("BBFFBBFRLL"));
    }
}