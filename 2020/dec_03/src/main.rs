mod constants;

fn check_slopes(row_inc: usize, column_inc: usize) -> i128 {
    let mut row = 0;
    let mut column = 0;
    let mut result = 0;
    while row + row_inc < constants::INPUT.len(){
        row += row_inc; 
        column = (column + column_inc) % constants::INPUT[0].len();
        let char_in_string = constants::INPUT[row].chars().nth(column);
        match char_in_string {
            Some('#') => result += 1,
            Some('.') => (),
            _         => panic!("Char should be known: row: {}, column: {}", row, column),
        }
    }

    return result;
}

fn main() {

    println!("Solution 1: {}", check_slopes(1, 3));

    let mut result = check_slopes(1, 1);
    result *= check_slopes(1, 3);
    result *= check_slopes(1, 5);
    result *= check_slopes(1, 7);
    result *= check_slopes(2, 1);

    println!("Solution 2: {}", result);
}
