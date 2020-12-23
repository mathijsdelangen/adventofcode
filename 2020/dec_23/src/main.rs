fn simulate_move(cups : &Vec<usize>, cup_idx: usize) -> Vec<usize> {
    let mut new_cups = cups.to_owned();
    let mut removed_cups : Vec<usize> = Vec::new();
    let current_cup = new_cups[cup_idx];
    println!("Current cup: {}", current_cup);
    
    // Remove 3 cups after cup_idx
    for _ in 0..3 {
        if cup_idx < new_cups.len() {
            println!(" Remove cup {} at index {}", new_cups[(cup_idx+1) % new_cups.len()], (cup_idx+1) % new_cups.len());
            removed_cups.push(new_cups.remove((cup_idx+1) % new_cups.len())); 
        }    else {
            println!(" Remove cup {} at index {}", new_cups[0], 0);
            removed_cups.push(new_cups.remove(0));
        }
    }

    // Find destination cup
    let mut destination_cup = (current_cup+cups.len()) % (cups.len()+1);
    while !new_cups.contains(&destination_cup) {
        destination_cup = (destination_cup+cups.len()) % (cups.len()+1);
    }

    // Find index of destination cup
    let mut index = new_cups.iter().position(|&p| p == destination_cup).unwrap();
    println!("Insert at index {} ({})", index, new_cups[index]);
    // Add removed cups after destiation cup
    for cup in removed_cups {
        index += 1;
        new_cups.insert(index, cup);
    }

    return new_cups;
}

fn simulate_moves(starting_cups : &Vec<usize>, nr_moves: usize) -> Vec<usize> {
    println!("Simulate {} moves", nr_moves);
    let mut cups = starting_cups.to_owned();
    let mut current_cup = starting_cups[0];

    for _ in 0..nr_moves {
        let current_cup_index = cups.iter().position(|&p| p == current_cup).unwrap();
        cups = simulate_move(&cups, current_cup_index);

        let current_cup_index = cups.iter().position(|&p| p == current_cup).unwrap();
        current_cup = cups[(current_cup_index+1) % cups.len()];
    }
    return cups;
}

fn labels_to_solution(labels : &Vec<usize>) -> String {
    // Find index of label 1
    let index = labels.iter().position(|&p| p == 1).unwrap();
    let first_part = labels[index+1..].iter().map(|i| i.to_string()).collect::<String>();
    let second_part = labels[..index].iter().map(|i| i.to_string()).collect::<String>();

    return first_part + &second_part;
}

fn main() {
    let input : Vec<usize> = vec![2,5,3,1,4,9,8,6,7];
    let labels = simulate_moves(&input, 100);
    println!("Solution: {}", labels_to_solution(&labels));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_steps() {
        let input : Vec<usize> = vec![3,8,9,1,2,5,4,6,7];
        assert_eq!(vec![3,2,8,9,1,5,4,6,7], simulate_moves(&input, 1));
        assert_eq!(vec![3,2,5,4,6,7,8,9,1], simulate_moves(&input, 2));
        assert_eq!(vec![3,4,6,7,2,5,8,9,1], simulate_moves(&input, 3));
        assert_eq!(vec![4,6,7,9,1,3,2,5,8], simulate_moves(&input, 4));
        assert_eq!(vec![4,1,3,6,7,9,2,5,8], simulate_moves(&input, 5));
        assert_eq!(vec![4,1,9,3,6,7,2,5,8], simulate_moves(&input, 6));
    }

    #[test]
    fn validate_labels_to_solution() {
        let input : Vec<usize> = vec![3,8,9,1,2,5,4,6,7];
        assert_eq!("25467389", labels_to_solution(&input));
    }

    #[test]
    fn validate_example_10_moves() {
        let input : Vec<usize> = vec![3,8,9,1,2,5,4,6,7];
        let labels = simulate_moves(&input, 10);
        assert_eq!("92658374", labels_to_solution(&labels));
    }

    #[test]
    fn validate_example() {
        let input : Vec<usize> = vec![3,8,9,1,2,5,4,6,7];
        let labels = simulate_moves(&input, 100);
        assert_eq!("67384529", labels_to_solution(&labels));
    }
}