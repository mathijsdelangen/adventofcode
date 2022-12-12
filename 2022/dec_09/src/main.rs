use core::panic;
use std::fs;

type Move = (Direction, usize);
type Moves = Vec<Move>;

//type Position = (i32, i32);
type Positions = Vec<Position>;

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn from_string(direction: &str) -> Direction {
        match direction {
            "R" => return Direction::Right,
            "L" => return Direction::Left,
            "U" => return Direction::Up,
            "D" => return Direction::Down,
            _ => panic!("Unknown direction {:?}", direction),
        }
    }
}

fn parse_line(line: &str) -> Move {
    let parts = line.split_once(" ").unwrap();
    return (
        Direction::from_string(parts.0),
        parts.1.parse::<usize>().unwrap(),
    );
}

fn parse_input(input_file: &str) -> Moves {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    return input
        .split("\n")
        .map(|l| parse_line(l.trim()))
        .collect::<Vec<_>>();
}

fn knot_in_range(head_position: &Position, tail_position: &Position) -> bool {
    return (head_position.x - tail_position.x).abs() <= 1
        && (head_position.y - tail_position.y).abs() <= 1;
}

fn set_new_knot_position(rope: &mut Vec<Position>, knot_idx: usize) -> () {
    // On same line
    if rope[knot_idx].x == rope[knot_idx - 1].x || rope[knot_idx].y == rope[knot_idx - 1].y {
        if knot_in_range(
            &rope[knot_idx - 1],
            &Position {
                x: rope[knot_idx].x + 1,
                y: rope[knot_idx].y,
            },
        ) {
            rope[knot_idx].x += 1;
        } else if knot_in_range(
            &rope[knot_idx - 1],
            &Position {
                x: rope[knot_idx].x - 1,
                y: rope[knot_idx].y,
            },
        ) {
            rope[knot_idx].x -= 1;
        } else if knot_in_range(
            &rope[knot_idx - 1],
            &Position {
                x: rope[knot_idx].x,
                y: rope[knot_idx].y + 1,
            },
        ) {
            rope[knot_idx].y += 1
        } else if knot_in_range(
            &rope[knot_idx - 1],
            &Position {
                x: rope[knot_idx].x,
                y: rope[knot_idx].y - 1,
            },
        ) {
            rope[knot_idx].y -= 1
        }
    }
    // Diagonally
    else if knot_in_range(
        &rope[knot_idx - 1],
        &Position {
            x: rope[knot_idx].x + 1,
            y: rope[knot_idx].y + 1,
        },
    ) {
        rope[knot_idx].x += 1;
        rope[knot_idx].y += 1;
    } else if knot_in_range(
        &rope[knot_idx - 1],
        &Position {
            x: rope[knot_idx].x + 1,
            y: rope[knot_idx].y - 1,
        },
    ) {
        rope[knot_idx].x += 1;
        rope[knot_idx].y -= 1;
    } else if knot_in_range(
        &rope[knot_idx - 1],
        &Position {
            x: rope[knot_idx].x - 1,
            y: rope[knot_idx].y + 1,
        },
    ) {
        rope[knot_idx].x -= 1;
        rope[knot_idx].y += 1;
    } else if knot_in_range(
        &rope[knot_idx - 1],
        &Position {
            x: rope[knot_idx].x - 1,
            y: rope[knot_idx].y - 1,
        },
    ) {
        rope[knot_idx].x -= 1;
        rope[knot_idx].y -= 1;
    } else {
        panic!("We are too far out of range!");
    }
}

fn handle_knot(mut rope: &mut Vec<Position>, knot_idx: usize) -> bool {
    if !knot_in_range(&rope[knot_idx - 1], &rope[knot_idx]) {
        set_new_knot_position(&mut rope, knot_idx);
        return true;
    }
    return false;
}

fn walk(moves: &Moves, nr_knots: usize) -> Positions {
    let mut rope = vec![Position::default(); nr_knots];
    let mut positions_of_tail = Positions::new();
    let tail_idx = nr_knots - 1;

    positions_of_tail.push(rope[tail_idx]);

    for (direction, nr_moves) in moves {
        for _ in 0..*nr_moves {
            match direction {
                Direction::Right => rope[0].x += 1,
                Direction::Left => rope[0].x -= 1,
                Direction::Up => rope[0].y += 1,
                Direction::Down => rope[0].y -= 1,
            }
            //println!("Head at {:?}, tail at {:?}", rope[0], rope[tail_idx]);

            for knot_idx in 1..=nr_knots - 2 {
                if handle_knot(&mut rope, knot_idx) {
                    //println!("  Knot {:?} changed to position {:?}", knot_idx, rope[knot_idx]);
                }
            }

            // Handle tail seperately
            if handle_knot(&mut rope, tail_idx) {
                //println!("  New tail pos! {:?}", rope[tail_idx]);
                positions_of_tail.push(rope[tail_idx]);
            }
        }
    }

    return positions_of_tail;
}

fn num_positions_visited(positions: &Positions) -> usize {
    let mut check_pos = positions.to_vec();
    check_pos.sort_by(|a, b| {
        if a.y == b.y {
            a.x.cmp(&b.x)
        } else {
            a.y.cmp(&b.y)
        }
    });
    check_pos.dedup();
    return check_pos.len();
}

fn main() {
    let moves = parse_input("assets/input.in");

    let positions_two_knots = walk(&moves, 2);
    println!(
        "Solution 1: {:?}",
        num_positions_visited(&positions_two_knots)
    );

    let positions_ten_knots = walk(&moves, 10);
    println!(
        "Solution 2: {:?}",
        num_positions_visited(&positions_ten_knots)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_example_1() {
        assert_eq!(
            13,
            num_positions_visited(&walk(&parse_input("assets/example.in"), 2))
        );
    }

    #[test]
    fn validate_example_2a() {
        assert_eq!(
            1,
            num_positions_visited(&walk(&parse_input("assets/example.in"), 10))
        );
    }

    #[test]
    fn validate_example_2b() {
        assert_eq!(
            36,
            num_positions_visited(&walk(&parse_input("assets/example2.in"), 10))
        );
    }
}
