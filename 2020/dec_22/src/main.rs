fn combat(player_1_deck: Vec<u32>, player_2_deck: Vec<u32>) -> u32 {
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

    let winners_deck = if deck_1.len() > 0 {deck_1} else {deck_2};
    return winners_deck.iter().enumerate().map(|(i,v)| v*((winners_deck.len()-i) as u32)).sum();
}

fn main() {
    let player_1_deck = vec![19,22,43,38,23,21,2,40,31,17,27,28,35,44,41,47,50,7,39,5,42,25,33,3,48];
    let player_2_deck = vec![16,24,36,6,34,11,8,30,26,15,9,10,14,1,12,4,32,13,18,46,37,29,20,45,49];

    println!("Solution: {}", combat(player_1_deck, player_2_deck));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_example() {
        assert_eq!(306, combat(vec![9,2,6,3,1], vec![5,8,4,7,10]));
    }
}