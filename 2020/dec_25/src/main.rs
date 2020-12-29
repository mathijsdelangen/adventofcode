fn determine_loop_size(key: u128) -> u128 {
    let mut loop_size = 0;
    let subject_number = 7;
    let mut value = 1;
    loop {
        loop_size += 1;
        value = (subject_number * value) % 20201227;
        if value == key { return loop_size}
    }
}

fn determine_encryption_key(subject_number: u128, loop_size: u128) -> u128 {
    let mut value = 1;
    for _ in 0..loop_size {
        value = (subject_number * value) % 20201227;
    }
    return value;
}

fn main() {
    let public_key_door = 15113849;
    let public_key_card = 4206373;

    let loop_size_door = determine_loop_size(public_key_door);
    let loop_size_card = determine_loop_size(public_key_card);

    assert_eq!(determine_encryption_key(public_key_card, loop_size_door), determine_encryption_key(public_key_door, loop_size_card));

    println!("Solution: {}", determine_encryption_key(public_key_card, loop_size_door));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_example() {
        let public_key_door = 17807724;
        let public_key_card = 5764801;
    
        let loop_size_door = determine_loop_size(public_key_door);
        let loop_size_card = determine_loop_size(public_key_card);

        assert_eq!(8, loop_size_card);
        assert_eq!(11, loop_size_door);
        
        assert_eq!(14897079, determine_encryption_key(public_key_card, loop_size_door));
        assert_eq!(14897079, determine_encryption_key(public_key_door, loop_size_card));
    }
}