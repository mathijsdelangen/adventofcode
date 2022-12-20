use std::{fs, process::id};

struct Pair {
    Left: String,
    Right: String,
}

#[derive(Debug, PartialEq, Eq)]
enum Ordering {
    Correct,
    Incorrect,
    Undecided,
}

fn parse_pair(input_file: &str) -> Pair {
    let parts: Vec<&str> = input_file.split("\n").map(|l| l.trim()).collect();

    return Pair {
        Left: parts[0].to_string(),
        Right: parts[1].to_string(),
    };
}

fn parse_input(input_file: &str) -> Vec<Pair> {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    return input
        .split("\r\n\r\n")
        .map(|l| parse_pair(l.trim()))
        .collect::<Vec<_>>();
}

fn determine_closing_bracket_pos(line: &str) -> usize {
    let mut brackets_seen = 0;
    for (i, c) in line.chars().enumerate() {
        match c {
            '[' => brackets_seen += 1,
            ']' => {
                brackets_seen -= 1;
                if brackets_seen == 0 {
                    return i;
                }
            }
            _ => (),
        }
    }

    return line.len() - 1;
}

fn get_first_value(line: &str, start_index: usize) -> (usize, usize) {
    let next_index = line[start_index..]
        .find(",")
        .unwrap_or(line.len() - start_index)
        + start_index;
    let val = line[start_index..next_index]
        .to_string()
        .parse::<usize>()
        .unwrap();
    return (val, next_index - start_index + 1);
}

fn correct_ordering(left: &str, right: &str) -> Ordering {
    // println!(
    //     "Handle {:?} ({:?}) vs {:?} ({:?})",
    //     left,
    //     left.len(),
    //     right,
    //     right.len()
    // );

    if left.len() == 0 && right.len() > 0 {
        // println!("Left side out of items (1)");
        return Ordering::Correct;
    } else if right.len() == 0 && left.len() > 0 {
        // println!("Right side out of items (1)");
        return Ordering::Incorrect;
    }

    let mut ordering = Ordering::Undecided;

    let mut left_idx = 0;
    let mut right_idx = 0;

    while left_idx < left.len() && right_idx < right.len() && ordering == Ordering::Undecided {
        // println!("Move to left: {:?}, right: {:?}", left_idx, right_idx);
        // println!(
        //     "  Check first of {:?} vs {:?}",
        //     &left[left_idx..],
        //     &right[right_idx..]
        // );
        match (
            left.chars().nth(left_idx).unwrap(),
            right.chars().nth(right_idx).unwrap(),
        ) {
            ('[', '[') => {
                // Find index of ']' for both
                let left_pos = determine_closing_bracket_pos(&left[left_idx..]) + left_idx;
                let right_pos = determine_closing_bracket_pos(&right[right_idx..]) + right_idx;
                ordering = correct_ordering(
                    &left[left_idx + 1..left_pos],
                    &right[right_idx + 1..right_pos],
                );

                left_idx = left_pos + 2;
                right_idx = right_pos + 2;
            }
            ('[', _val_right) => {
                // Find index of ']' for left
                let left_pos = determine_closing_bracket_pos(&left[left_idx..]) + left_idx;

                let (_right_value, right_add_idx) = get_first_value(&right, right_idx);

                ordering = correct_ordering(
                    &left[left_idx + 1..left_pos],
                    &right[right_idx..right_idx + right_add_idx - 1],
                );

                left_idx = left_pos + 2;
                right_idx += right_add_idx;
            }
            (_val_left, '[') => {
                // Find index of ']' for right
                let right_pos = determine_closing_bracket_pos(&right[right_idx..]) + right_idx;

                let (_left_value, left_add_idx) = get_first_value(&left, left_idx);

                ordering = correct_ordering(
                    &left[left_idx..left_idx + left_add_idx - 1],
                    &right[right_idx + 1..right_pos],
                );

                left_idx += left_add_idx;
                right_idx = right_pos + 2;
            }
            _ => {
                let (left_value, left_add_idx) = get_first_value(&left, left_idx);
                let (right_value, right_add_idx) = get_first_value(&right, right_idx);

                // println!("  Values {:?} vs {:?}", left_value, right_value);

                if left_value == right_value {
                    left_idx += left_add_idx;
                    right_idx += right_add_idx;
                } else if left_value < right_value {
                    return Ordering::Correct;
                } else {
                    return Ordering::Incorrect;
                }
            }
        }
    }

    if ordering == Ordering::Undecided {
        // Check if we exceeded some list
        if right_idx >= right.len() && left_idx < left.len() {
            // println!("Right side out of items (2)");
            return Ordering::Incorrect;
        } else if left_idx >= left.len() && right_idx < right.len() {
            // println!("Left side out of items (2)");
            return Ordering::Correct;
        }
    }

    return ordering;
}

fn pairs_in_correct_order(pairs: &Vec<Pair>) -> usize {
    let mut sum_of_indices = 0;
    for (idx, pair) in pairs.iter().enumerate() {
        if correct_ordering(&pair.Left, &pair.Right) == Ordering::Correct {
            sum_of_indices += idx + 1;
            // println!("Correct order");
        } else {
            // println!("Incorrect order");
        }
    }

    return sum_of_indices;
}

fn insert_sorted<'a>(packets: &mut Vec<&'a str>, packet: &'a str) {
    let mut insert_at = packets.len();

    if packets.len() == 0 {
        insert_at = 0
    } else if packets.len() == 1 {
        if correct_ordering(&packets[0], packet) == Ordering::Correct {
            insert_at = 1;
        } else {
            insert_at = 0;
        }
    } else {
        if correct_ordering(packet, &packets[0]) == Ordering::Correct {
            insert_at = 0;
        } else {
            for i in 0..packets.len() - 1 {
                if correct_ordering(&packets[i], packet) == Ordering::Correct
                    && correct_ordering(packet, &packets[i + 1]) == Ordering::Correct
                {
                    insert_at = i + 1;
                    break;
                }
            }
        }
    }
    // println!("Insert {:?} at location {:?}", packet, insert_at);
    packets.insert(insert_at, packet);
    // println!("Packets contains: \n {:?}", packets);
}

fn divider_packet_locations(pairs: &Vec<Pair>) -> usize {
    let mut all_packets = Vec::new();

    for pair in pairs {
        insert_sorted(&mut all_packets, &pair.Left);
        insert_sorted(&mut all_packets, &pair.Right);
    }

    let div_2 = String::from("[[2]]");
    insert_sorted(&mut all_packets, &div_2);

    let div_6 = String::from("[[6]]");
    insert_sorted(&mut all_packets, &div_6);

    println!("{:?}", all_packets);

    return (all_packets.iter().position(|&r| r == div_2).unwrap() + 1)
        * (all_packets.iter().position(|&r| r == div_6).unwrap() + 1);
}

fn main() {
    let pairs = parse_input("assets/input.in");
    println!("Solution 1: {:?}", &pairs_in_correct_order(&pairs));
    println!("Solution 2: {:?}", &divider_packet_locations(&pairs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_example_1() {
        let input = parse_input("assets/example.in");
        assert_eq!(13, pairs_in_correct_order(&input));
    }

    #[test]
    fn validate_correct_ordering() {
        assert_eq!(
            Ordering::Correct,
            correct_ordering(&String::from("[]"), &String::from("[[]]"))
        );

        assert_eq!(
            Ordering::Correct,
            correct_ordering(&String::from("[[]]"), &String::from("[[[]]]"))
        );

        assert_eq!(
            Ordering::Correct,
            correct_ordering(&String::from("[[1]]"), &String::from("[[2]]"))
        );

        assert_eq!(
            Ordering::Correct,
            correct_ordering(&String::from("[1,2]"), &String::from("[1,2,3]"))
        );

        assert_eq!(
            Ordering::Correct,
            correct_ordering(&String::from("[10,8]"), &String::from("[10,8,3]"))
        );

        assert_eq!(
            Ordering::Correct,
            correct_ordering(&String::from("[]"), &String::from("[9]"))
        );
    }

    #[test]
    fn validate_incorrect_ordering() {
        assert_eq!(
            Ordering::Incorrect,
            correct_ordering(
                &String::from("[[1,2,3,4,5],1]"),
                &String::from("[1,2,3,4,5],6")
            ) // [1,2,3,4,5] vs 1,2,3,4,5
              // [1,2,3,4,5] vs [1]
        );

        assert_eq!(
            Ordering::Incorrect,
            correct_ordering(&String::from("[9]"), &String::from("[]"))
        );
    }

    #[test]
    fn validate_input_1_fail() {
        let input = parse_input("assets/input.in");
        assert!(pairs_in_correct_order(&input) < 5143);
        assert!(pairs_in_correct_order(&input) < 5063);
        assert!(pairs_in_correct_order(&input) == 4894);
    }

    #[test]
    fn validate_example_2() {
        let input = parse_input("assets/example.in");
        assert_eq!(140, divider_packet_locations(&input));
    }
}
