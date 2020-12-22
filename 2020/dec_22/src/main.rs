fn determine_score(winners_deck: &Vec<u32>) -> u32 {
    return winners_deck.iter().enumerate().map(|(i,v)| v*((winners_deck.len()-i) as u32)).sum();
}

fn combat(player_1_deck: &Vec<u32>, player_2_deck: &Vec<u32>) -> (u32, Vec<u32>) {
    let mut deck_1 = player_1_deck.to_owned();
    let mut deck_2 = player_2_deck.to_owned();
    while deck_1.len() > 0 && deck_2.len() > 0 {
        let player_1_card = deck_1.remove(0);
        let player_2_card = deck_2.remove(0);
        if player_1_card > player_2_card {
            deck_1.push(player_1_card);
            deck_1.push(player_2_card);
        } else {
            deck_2.push(player_2_card);
            deck_2.push(player_1_card);
        }
    }

    if deck_1.len() > 0 {return (1, deck_1)} else {return (2, deck_2)}
}

fn recursive_combat(player_1_deck: &Vec<u32>, player_2_deck: &Vec<u32>) -> (u32, Vec<u32>) {
    let mut deck_1 = player_1_deck.to_owned();
    let mut deck_2 = player_2_deck.to_owned();
    let mut known_configurations_deck_1 : Vec<Vec<u32>> = Vec::new();
    let mut known_configurations_deck_2 : Vec<Vec<u32>> = Vec::new();

    while deck_1.len() > 0 && deck_2.len() > 0 {
        if known_configurations_deck_1.contains(&deck_1) && known_configurations_deck_2.contains(&deck_2){
            // Loop detected
            return (1, deck_1);
        }
        known_configurations_deck_1.push(deck_1.to_owned());
        known_configurations_deck_2.push(deck_2.to_owned());

        let player_1_card = deck_1.remove(0);
        let player_2_card = deck_2.remove(0);

        if player_1_card <= deck_1.len() as u32 && 
           player_2_card <= deck_2.len() as u32 
        {
            if recursive_combat(&deck_1[0..player_1_card as usize].to_vec(), &deck_2[0..player_2_card as usize].to_vec()).0 == 1 {
                deck_1.push(player_1_card);
                deck_1.push(player_2_card);
            } else {
                deck_2.push(player_2_card);
                deck_2.push(player_1_card);
            }
        }
        else if player_1_card > player_2_card {
            deck_1.push(player_1_card);
            deck_1.push(player_2_card);
        } else {
            deck_2.push(player_2_card);
            deck_2.push(player_1_card);
        }
    }

    if deck_1.len() > 0 {return (1, deck_1)} else {return (2, deck_2)}
}

fn main() {
    let player_1_deck = vec![19,22,43,38,23,21,2,40,31,17,27,28,35,44,41,47,50,7,39,5,42,25,33,3,48];
    let player_2_deck = vec![16,24,36,6,34,11,8,30,26,15,9,10,14,1,12,4,32,13,18,46,37,29,20,45,49];

    println!("Solution 1: {}", determine_score(&combat(&player_1_deck, &player_2_deck).1));
    println!("Solution 2: {}", determine_score(&recursive_combat(&player_1_deck, &player_2_deck).1));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_example() {
        assert_eq!(306, determine_score(&combat(&vec![9,2,6,3,1], &vec![5,8,4,7,10]).1));
    }

    #[test]
    fn validate_example2() {
        assert_eq!(291, determine_score(&recursive_combat(&vec![9,2,6,3,1], &vec![5,8,4,7,10]).1));
    }

    #[test]
    fn validate_no_infinite_loop() {
        assert_eq!(105, determine_score(&recursive_combat(&vec![43,19], &vec![2,29,14]).1));
    }
}