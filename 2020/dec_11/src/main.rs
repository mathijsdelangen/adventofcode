use std::fs;

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

            let mut local_seats = Vec::new();
            for row in seating[row_low..=row_high].iter() {
                let mut local_column = Vec::new();
                    for column in row[col_low..=col_high].iter() {
                    local_column.push(*column);
                }
               local_seats.push(local_column);
            }
            match seating[row_idx][col_idx] {
                '#' => 
                    {
                        if calculate_occupied_seats(&local_seats) >= 4 + 1 { seats.push((row_idx,col_idx)); }
                    },
                'L' => 
                    {
                        if calculate_occupied_seats(&local_seats) == 0 { seats.push((row_idx,col_idx)); }
                    },
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
    return nr_occupied_seats;
}

fn find_stable_seating(seating: Vec<Vec<char>>) -> u32 {

    let mut seats = seating.to_owned();
    let mut occupied_seats = 0;
    loop {
        let seats_to_change = find_seats_to_change(&seats);
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
    let input_lines = parse_input(input_file);

    println!("Solution 1: {}", find_stable_seating(input_lines));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_example() {
        assert_eq!(37, find_stable_seating(parse_input("assets/dec_11_example.in")));
        assert_eq!(26, find_stable_seating_sol2(parse_input("assets/dec_11_example.in")));
    }

    #[test]
    fn validate_calculate_occcupied_seats() {
        assert_eq!(37, calculate_occupied_seats(&parse_input("assets/dec_11_example_solution1.in")));
    }
}