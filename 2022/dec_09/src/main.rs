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

fn tail_in_range(head_position: &Position, tail_position: &Position) -> bool {
    return (head_position.x - tail_position.x).abs() <= 1
        && (head_position.y - tail_position.y).abs() <= 1;
}

fn set_new_tail_position(head_position: &Position, tail_position: &mut Position) -> () {
    // On same line
    if tail_position.x == head_position.x || tail_position.y == head_position.y {
        if tail_in_range(
            head_position,
            &Position {
                x: tail_position.x + 1,
                y: tail_position.y,
            },
        ) {
            tail_position.x += 1;
        } 
        else if tail_in_range(
            head_position,
            &Position {
                x: tail_position.x - 1,
                y: tail_position.y,
            },
        ) {
            tail_position.x -= 1;
        } else if tail_in_range(
            head_position,
            &Position {
                x: tail_position.x,
                y: tail_position.y + 1,
            },
        ) {
            tail_position.y += 1
        }
        else if tail_in_range(
            head_position,
            &Position {
                x: tail_position.x,
                y: tail_position.y - 1,
            },
        ) {
            tail_position.y -= 1
        }
    }
    // Diagonally
    else if tail_in_range(
        head_position,
        &Position {
            x: tail_position.x + 1,
            y: tail_position.y + 1,
        },
    ) {
        tail_position.x += 1;
        tail_position.y += 1;
    } else if tail_in_range(
        head_position,
        &Position {
            x: tail_position.x + 1,
            y: tail_position.y - 1,
        },
    ) {
        tail_position.x += 1;
        tail_position.y -= 1;
    } else if tail_in_range(
        head_position,
        &Position {
            x: tail_position.x - 1,
            y: tail_position.y + 1,
        },
    ) {
        tail_position.x -= 1;
        tail_position.y += 1;
    } else if tail_in_range(
        head_position,
        &Position {
            x: tail_position.x - 1,
            y: tail_position.y - 1,
        },
    ) {
        tail_position.x -= 1;
        tail_position.y -= 1;
    } else {
        panic!("We are too far out of range!");
    }
}

fn handle_tail(head_position: &Position, mut tail_position: &mut Position) -> bool {
    if !tail_in_range(&head_position, &tail_position) {
        set_new_tail_position(head_position, &mut tail_position);
        return true;
    }
    return false;
}

fn walk(moves: &Moves) -> Positions {
    let mut head_pos = Position::default();
    let mut tail_pos = Position::default();
    let mut positions = Positions::new();
    positions.push(tail_pos);

    for (direction, nr_moves) in moves {
        for _ in 0..*nr_moves {
            match direction {
                Direction::Right => head_pos.x += 1,
                Direction::Left => head_pos.x -= 1,
                Direction::Up => head_pos.y += 1,
                Direction::Down => head_pos.y -= 1,
                _ => panic!("Unknown move"),
            }
            //println!("Head at {:?}, tail at {:?}", head_pos, tail_pos);

            if handle_tail(&head_pos, &mut tail_pos) {
                // println!("  New tail pos! {:?}", tail_pos);
                positions.push(tail_pos);
            }
        }
    }

    return positions;
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
    let positions = walk(&moves);
    println!("Solution 1: {:?}", num_positions_visited(&positions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_example_1() {
        assert_eq!(
            13,
            num_positions_visited(&walk(&parse_input("assets/example.in")))
        );
    }

    #[test]
    fn validate_example_2() {}
}
