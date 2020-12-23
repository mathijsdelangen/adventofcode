fn calculate_destination_cup(cup: usize, max_element: usize) -> usize {
    assert_ne!(cup, 0);
    return if cup == 1 { max_element } else { cup-1 };
}

fn simulate_move(cups : &mut Vec<usize>, current_cup: &mut usize) {
    // Store the three elements next to the current cup
    let next_cup_1 = cups[*current_cup];
    let next_cup_2 = cups[next_cup_1];
    let next_cup_3 = cups[next_cup_2];
    let new_neighbor = cups[next_cup_3];

    // Determine destination cup
    let mut destination_cup = calculate_destination_cup(*current_cup, cups.len()-1);
    while destination_cup == next_cup_1 || destination_cup == next_cup_2 || destination_cup == next_cup_3 {
        destination_cup = calculate_destination_cup(destination_cup, cups.len()-1);
    }

    // Update current cup
    cups[*current_cup] = new_neighbor;

    // Update destination cup
    let dest_neighbor = cups[destination_cup];
    cups[destination_cup] = next_cup_1;
    cups[next_cup_3] = dest_neighbor;

    // Update current cup
    *current_cup = new_neighbor;
}

fn from_input_to_cups(starting_cups : &Vec<usize>) -> Vec<usize> {
    // We store the right neigbor for each cup. 
    // So for input [1,4,2,3] this means 
    // 1 points to 4, 
    // 2 points to 3,
    // 3 points to 1, 
    // 4 points to 2, 
    // resulting in the following array:
    // [0,4,3,1,2]
    // (we keep the zero for easier indexing)

    let mut cups = vec![0;starting_cups.len()+1];
    for i in 0..starting_cups.len()-1 {
        cups[starting_cups[i]] = starting_cups[i+1];
    }
    cups[starting_cups[starting_cups.len()-1]] = starting_cups[0];
    return cups;
}

fn simulate_moves(starting_cups : &Vec<usize>, nr_moves: usize) -> Vec<usize> {
    println!("Simulate {} moves", nr_moves);

    let mut current_cup = starting_cups[0];
    let mut cups = from_input_to_cups(&starting_cups.to_owned());

    for _ in 0..nr_moves {
        simulate_move(&mut cups, &mut current_cup);
    }
    return cups;
}

fn ordered_cups(cups : &Vec<usize>) -> Vec<usize> {
    let mut ordered_values = Vec::new();
    let mut c = cups[1];
    loop {
        if c == 1 { break; }
        ordered_values.push(c);
        c = cups[c];
    }
    return ordered_values;
}

fn ordered_cups_to_string(cups : &Vec<usize>) -> String {
    return ordered_cups(cups).iter().map(|i| i.to_string()).collect::<String>();
}

fn solution2_from_result(cups : &Vec<usize>) -> u64 {
    return (cups[1]*cups[cups[1]]) as u64;
}

fn main() {
    let mut input : Vec<usize> = Vec::from(vec![2,5,3,1,4,9,8,6,7]);
    println!("Solution 1: {}", ordered_cups_to_string(&simulate_moves(&input, 100)));

    // Extend list for part 2
    let rest_of_inputs : Vec<usize> = (10..=1_000_000).map(|x| x).collect();
    input.extend(rest_of_inputs);
    println!("Solution 2: {}", solution2_from_result(&simulate_moves(&input, 10_000_000)));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_steps() {
        let input : Vec<usize> = Vec::from(vec![3,8,9,1,2,5,4,6,7]);
        assert_eq!("54673289", ordered_cups_to_string(&simulate_moves(&input, 1)));
        assert_eq!("32546789", ordered_cups_to_string(&simulate_moves(&input, 2)));
        assert_eq!("34672589", ordered_cups_to_string(&simulate_moves(&input, 3)));
        assert_eq!("32584679", ordered_cups_to_string(&simulate_moves(&input, 4)));
        assert_eq!("36792584", ordered_cups_to_string(&simulate_moves(&input, 5)));
        assert_eq!("93672584", ordered_cups_to_string(&simulate_moves(&input, 6)));
    }

    #[test]
    fn validate_example_10_moves() {
        let input : Vec<usize> = Vec::from(vec![3,8,9,1,2,5,4,6,7]);
        assert_eq!("92658374", ordered_cups_to_string(&simulate_moves(&input, 10)));
    }

    #[test]
    fn validate_example() {
        let input : Vec<usize> = Vec::from(vec![3,8,9,1,2,5,4,6,7]);
        assert_eq!("67384529", ordered_cups_to_string(&simulate_moves(&input, 100)));
    }

    #[test]
    fn validate_example2() {
        let mut input : Vec<usize> = Vec::from(vec![3,8,9,1,2,5,4,6,7]);
        let rest_of_inputs : Vec<usize> = (10..=1_000_000).map(|x| x).collect();
        input.extend(rest_of_inputs);

        assert_eq!(149245887792, solution2_from_result(&simulate_moves(&input, 10_000_000)));
    }
}