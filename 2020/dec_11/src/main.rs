use std::fs;

#[allow(dead_code)]
fn print_seating(seating: &Vec<Vec<char>>) {
    for row in seating {
        for c in row {
            print!("{}",c);
        }
        println!();
    }
}

fn parse_input(input_file: &str) -> Vec<Vec<char>> {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let input_lines : Vec<Vec<char>> = input.split("\n")
                                        .map(|v| v.trim())
                                        .map(|s| s.chars().collect())
                                        .collect();
    return input_lines;
}

fn change_seats(original_seating: Vec<Vec<char>>, seats_to_change: Vec<(usize,usize)>) ->  Vec<Vec<char>> {

    let mut new_seating = original_seating.to_owned();
    for (seat_row, seat_column) in seats_to_change.iter() {
        let row  = *seat_row;
        let column = *seat_column;
        match original_seating[row][column] {
            'L' => new_seating[row][column] = '#',
            '#' => new_seating[row][column] = 'L',
            _   => panic!("Seat at ({},{}) cannot be changed", seat_column, seat_row),
        }
    }
    return new_seating;
}

fn find_seats_to_change(seating: &Vec<Vec<char>>) -> Vec<(usize,usize)> {

    let mut seats = Vec::new();

    for row_idx in 0..seating.len() {
        for col_idx in 0..seating[row_idx].len() {
            let row_low = std::cmp::max((row_idx as i32)-1,0) as usize;
            let row_high = std::cmp::min(row_idx+1,seating.len()-1);
            let col_low = std::cmp::max((col_idx as i32)-1,0) as usize;
            let col_high = std::cmp::min(col_idx+1,seating[row_idx].len()-1);

            let mut seats_to_check = Vec::new();
            for row in seating[row_low..=row_high].iter() {
                let mut local_column = Vec::new();
                    for column in row[col_low..=col_high].iter() {
                    local_column.push(*column);
                }
               seats_to_check.push(local_column);
            }
            match seating[row_idx][col_idx] {
                '#' => 
                    {
                        if calculate_occupied_seats(&seats_to_check) >= 4 + 1 { seats.push((row_idx,col_idx)); }
                    },
                'L' => 
                    {
                        if calculate_occupied_seats(&seats_to_check) == 0 { seats.push((row_idx,col_idx)); }
                    },
                _   => (),
            }
        }
    }

    return seats;
}

fn calculate_nr_seats_in_view((seat_row, seat_col) : (usize,usize), seating: &Vec<Vec<char>>) -> u32 {

    let mut nr_seats_in_view = 0;
  
    // Horizontal
    for column_idx in (0..seat_col).rev() {
        let c = seating[seat_row][column_idx];
        if c == '#' {nr_seats_in_view += 1}
        if c == 'L' || c == '#' { break; }
    }
    for column_idx in seat_col+1..seating[seat_row].len() {
        let c = seating[seat_row][column_idx];
        if c == '#' {nr_seats_in_view += 1; break}
        if c == 'L' || c == '#' { break; }
    }

    // Vertical
    for row_idx in (0..seat_row).rev() {
        let c = seating[row_idx][seat_col];
        if c == '#' {nr_seats_in_view += 1; break}
        if c == 'L' || c == '#' { break; }
    }
    
    for row_idx in seat_row+1..seating.len() {
        let c = seating[row_idx][seat_col];
        if c == '#' {nr_seats_in_view += 1; break}
        if c == 'L' || c == '#' { break; }
    }

    // Diagonal
    for d in 1..=std::cmp::min(seat_row,seat_col) {
        //println!("Checking ({},{})",seat_row-d,seat_col-d);
        let c = seating[seat_row-d][seat_col-d];
        if c == '#' {nr_seats_in_view += 1; break}
        if c == 'L' || c == '#' { break; }
    }
    
    for d in 1..=std::cmp::min((seating.len()-1)-seat_row,(seating[0].len()-1)-seat_col) {
        //println!("Checking ({},{})",seat_row+d,seat_col+d);
        let c = seating[seat_row+d][seat_col+d];
        if c == '#' {nr_seats_in_view += 1; break}
        if c == 'L' || c == '#' { break; }
    }

    for d in 1..=std::cmp::min((seating.len()-1)-seat_row,seat_col) {
        //println!("Checking ({},{})",seat_row+d,seat_col-d);
        let c = seating[seat_row+d][seat_col-d];
        if c == '#' {nr_seats_in_view += 1; break}
        if c == 'L' || c == '#' { break; }
    }

    for d in 1..=std::cmp::min(seat_row,(seating[0].len()-1)-seat_col) {
        //println!("Checking ({},{})",seat_row-d,seat_col+d);
        let c = seating[seat_row-d][seat_col+d];
        if c == '#' {nr_seats_in_view += 1; break}
        if c == 'L' || c == '#' { break; }
    }
    
    //println!("Number of seats in view: {}", nr_seats_in_view);
    return nr_seats_in_view;
}

fn find_seats_to_change_in_view(seating: &Vec<Vec<char>>) -> Vec<(usize,usize)> {
    let mut seats = Vec::new();

    for row_idx in 0..seating.len() {
        for col_idx in 0..seating[row_idx].len() {
            //println!("Checking {},{} with value {}", row_idx, col_idx, seating[row_idx][col_idx]);
            match seating[row_idx][col_idx] {
                '#' => if calculate_nr_seats_in_view((row_idx,col_idx), seating) >= 5 { seats.push((row_idx,col_idx)); },
                'L' => if calculate_nr_seats_in_view((row_idx,col_idx), seating) == 0 { seats.push((row_idx,col_idx)); },
                _   => (),
            }
        }
    }
    return seats;
}

fn calculate_occupied_seats(seating: &Vec<Vec<char>>) -> u32 {

    let mut nr_occupied_seats = 0;
    for row_of_seats in seating.iter() {
        for seat in row_of_seats.iter() {
            match seat {
                'L' | '.' => (),
                '#'       => nr_occupied_seats +=1,
                _         => panic!("Seat indicated with '{}' unknown", seat),
            }
        }
    }
    //println!("Occupied seats: {}", nr_occupied_seats);
    return nr_occupied_seats;
}

fn find_stable_seating(seating: Vec<Vec<char>>, use_view : bool) -> u32 {

    let mut seats = seating.to_owned();
    let mut occupied_seats = 0;
    loop {
        let seats_to_change = if !use_view { find_seats_to_change(&seats) } else {find_seats_to_change_in_view(&seats) };
        println!("Found {} seats to change", seats_to_change.len());
        
        if seats_to_change.len() == 0 { break; }
        seats = change_seats(seats, seats_to_change);

        occupied_seats = calculate_occupied_seats(&seats);
        
        println!("Found {} occupied seats", occupied_seats);
    }
    return occupied_seats;
}

fn main() {
    let input_file = "assets/dec_11.in";

    println!("Solution 1: {}", find_stable_seating(parse_input(input_file), false));
    println!("Solution 2: {}", find_stable_seating(parse_input(input_file), true));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_example() {
        assert_eq!(37, find_stable_seating(parse_input("assets/dec_11_example.in"), false));
        assert_eq!(26, find_stable_seating(parse_input("assets/dec_11_example.in"), true));
    }

    #[test]
    fn validate_calculate_occcupied_seats() {
        assert_eq!(37, calculate_occupied_seats(&parse_input("assets/dec_11_example_solution1.in")));
    }

    #[test]
    fn validate_calculate_nr_seats_in_view() {
        assert_eq!(8, calculate_nr_seats_in_view((4,3),&parse_input("assets/dec_11_example_solution2a.in")));
        assert_eq!(0, calculate_nr_seats_in_view((3,3), &parse_input("assets/dec_11_example_solution2b.in")));
        assert_eq!(0, calculate_nr_seats_in_view((1,0), &parse_input("assets/dec_11_example_solution2c.in")));
    }

    #[test]
    fn validate_find_seats_to_change_in_view() {
        assert_eq!(1, find_seats_to_change_in_view(&parse_input("assets/dec_11_example_solution2a.in")).len());
        assert_eq!(17, find_seats_to_change_in_view(&parse_input("assets/dec_11_example_solution2b.in")).len());
    }

    #[test]
    fn validate_sol2_iteration_one() {
        let nr_seats = calculate_occupied_seats(&parse_input("assets/dec_11_example_solution2_after_one_iteration.in"));
        let input = parse_input("assets/dec_11_example.in");
        let seats_to_change = find_seats_to_change_in_view(&input);
        let new_seats = change_seats(input, seats_to_change);
        assert_eq!(nr_seats, calculate_occupied_seats(&new_seats));
    }
    
    #[test]
    fn validate_sol2_iteration_two() {
        let nr_seats = calculate_occupied_seats(&parse_input("assets/dec_11_example_solution2_after_two_iterations.in"));
        let input = parse_input("assets/dec_11_example_solution2_after_one_iteration.in");
        let seats_to_change = find_seats_to_change_in_view(&input);
        let new_seats = change_seats(input, seats_to_change);
        assert_eq!(nr_seats, calculate_occupied_seats(&new_seats));
    }

    #[test]
    fn validate_sol2_iteration_three() {
        let nr_seats = calculate_occupied_seats(&parse_input("assets/dec_11_example_solution2_after_three_iterations.in"));
        let input = parse_input("assets/dec_11_example_solution2_after_two_iterations.in");
        calculate_occupied_seats(&parse_input("assets/dec_11_example_solution2_after_two_iterations.in"));
        let seats_to_change = find_seats_to_change_in_view(&input);
        let new_seats = change_seats(input, seats_to_change);
        assert_eq!(nr_seats, calculate_occupied_seats(&new_seats));
    }
}