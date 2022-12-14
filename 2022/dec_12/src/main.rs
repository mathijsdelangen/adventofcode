use std::{fs, thread};

type Point = (usize, usize);

type Height = usize;
type Map = Vec<Vec<Height>>;

const INDICATE_S: Height = 100;
const INDICATE_E: Height = 200;

fn get_heigth(c: char) -> usize {
    if c.is_lowercase() {
        return c as usize - 'a' as usize;
    } else {
        match c {
            'S' => INDICATE_S,
            'E' => INDICATE_E,
            _ => panic!("Unrecognized location {:?}", c),
        }
    }
}

fn parse_input(input_file: &str) -> (Map, (usize, usize), (usize, usize)) {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");

    // Store map heights
    let mut map = input
        .lines()
        .map(|l| l.chars().map(|c| get_heigth(c)).collect())
        .collect::<Vec<Vec<_>>>();

    // Find starting point
    let mut starting_point = (0, 0);
    let mut end_point = (0, 0);
    for (y, line) in map.iter().enumerate() {
        for (x, height) in line.iter().enumerate() {
            if *height == INDICATE_S {
                starting_point = (x, y);
            }
            if *height == INDICATE_E {
                end_point = (x, y);
            }
        }
    }

    map[starting_point.1][starting_point.0] = get_heigth('a');
    map[end_point.1][end_point.0] = get_heigth('z');

    return (map, starting_point, end_point);
}

fn calculate_paths(
    height_map: &Map,
    distance_map: &mut Map,
    current_point: Point,
    current_distance: usize,
) -> () {
    let known_distance = distance_map[current_point.1][current_point.0];
    let height = height_map[current_point.1][current_point.0];

    // We already found a shorter path from this point
    if current_distance >= known_distance {
        return;
    }

    distance_map[current_point.1][current_point.0] = current_distance;

    let mut y_range: Vec<isize> = Vec::new();
    if current_point.1 == 0 {
        y_range.push(1);
    } else if current_point.1 == height_map.len() - 1 {
        y_range.push(-1);
    } else {
        y_range.push(-1);
        y_range.push(1);
    }

    let mut x_range: Vec<isize> = Vec::new();
    if current_point.0 == 0 {
        x_range.push(1);
    } else if current_point.0 == height_map[0].len() - 1 {
        x_range.push(-1);
    } else {
        x_range.push(-1);
        x_range.push(1);
    }

    for add_x in x_range {
        let x = (current_point.0 as isize + add_x) as usize;

        if height_map[current_point.1][x] + 1 >= height {
            calculate_paths(
                height_map,
                distance_map,
                (x, current_point.1),
                current_distance + 1,
            );
        }
    }
    for add_y in y_range {
        let y = (current_point.1 as isize + add_y) as usize;

        if height_map[y][current_point.0] + 1 >= height {
            calculate_paths(
                height_map,
                distance_map,
                (current_point.0, y),
                current_distance + 1,
            );
        }
    }
}

fn shortest_path_length(height_map: &Map, starting_point: &Point, end_point: &Point) -> Map {
    let mut distance_map = vec![vec![usize::MAX; height_map[0].len()]; height_map.len()];

    calculate_paths(height_map, &mut distance_map, *end_point, 0);

    return distance_map
}

fn calculcate_solution() -> () {
    let (map, starting_point, end_point) = parse_input("assets/input.in");

    let distance_map = shortest_path_length(&map, &starting_point, &end_point);
    let path_length = distance_map[starting_point.1][starting_point.0];

    println!(
        "Solution 1: {:?}",
        path_length
    );

    let mut shortest_path = path_length; // We already know this as a maximum

    for (y, line) in map.iter().enumerate() {
        for (x, height) in line.iter().enumerate() {
            if map[y][x] == 0 && distance_map[y][x] < shortest_path {
                shortest_path = distance_map[y][x];
            }
        }
    }

    println!(
        "Solution 2: {:?}",
        shortest_path
    );
    
}

fn main() {
    // Spawn thread with explicit stack size
    let child = thread::Builder::new()
        .stack_size(200 * 1024 * 1024)
        .spawn(calculcate_solution)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_example_1() {
        let (map, starting_point, end_point) = parse_input("assets/example.in");
        assert_eq!(31, shortest_path_length(&map, &starting_point, &end_point));
    }
}
