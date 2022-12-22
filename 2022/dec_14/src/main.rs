use std::fs;

type Map = Vec<Vec<Option<Material>>>;
#[derive(Clone, Debug, PartialEq, Eq)]
enum Material {
    Rock,
    Sand,
}

type Trace = Vec<Point>;
struct Point {
    x: usize,
    y: usize,
}

fn print_map(map: &Map) -> () {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            match &map[y][x] {
                Some(mat) => {
                    if *mat == Material::Rock {
                        print!("#");
                    } else {
                        print!("o");
                    }
                }
                None => print!("."),
            }
        }
        println!();
    }
}

fn parse_trace(trace: &str) -> Trace {
    let values = trace.split(" -> ").collect::<Vec<_>>();
    let mut pared_trace = Vec::new();
    for val in values {
        let (x, y) = val
            .split_once(",")
            .map(|s| {
                (
                    s.0.trim().parse::<usize>().unwrap(),
                    s.1.trim().parse::<usize>().unwrap(),
                )
            })
            .unwrap();
        pared_trace.push(Point { x, y });
    }
    return pared_trace;
}

fn parse_input(input_file: &str) -> (Map, Point) {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");

    let traces = input.split("\r\n").collect::<Vec<_>>();

    // Find min and max values
    let mut min_x = 500;
    let mut max_x = 500;
    let mut max_y = 0;

    for trace in traces.to_vec() {
        let parsed_trace = parse_trace(&trace);
        for p in parsed_trace {
            min_x = std::cmp::min(min_x, p.x);
            max_x = std::cmp::max(max_x, p.x);
            max_y = std::cmp::max(max_y, p.y);
        }
    }

    // println!("Min x: {:?}, max y: {:?}, max y: {:?}", min_x, max_x, max_y);

    // Create map, with a border around it (left, right and bottom)
    let y_border = 2;
    let y_length = max_y + 1 + y_border;

    let x_increase_border_size = y_length + 1;
    let x_length = (max_x - min_x) + 1 + 2 * x_increase_border_size;
    let mut map: Map = vec![vec![None; x_length]; y_length];

    for trace in traces.to_vec() {
        let parsed_trace = parse_trace(&trace);
        for val_idx in 0..parsed_trace.len() - 1 {
            let from = &parsed_trace[val_idx];
            let to = &parsed_trace[val_idx + 1];

            // Just go through all possible ways here (up, down, left, right)
            for y in from.y..=to.y {
                for x in from.x - min_x..=to.x - min_x {
                    map[y][x + x_increase_border_size] = Some(Material::Rock);
                }
                for x in to.x - min_x..=from.x - min_x {
                    map[y][x + x_increase_border_size] = Some(Material::Rock);
                }
            }
            for y in to.y..=from.y {
                for x in from.x - min_x..=to.x - min_x {
                    map[y][x + x_increase_border_size] = Some(Material::Rock);
                }
                for x in to.x - min_x..=from.x - min_x {
                    map[y][x + x_increase_border_size] = Some(Material::Rock);
                }
            }
        }
    }

    for x in 0..x_length {
        map[y_length - 1][x] = Some(Material::Rock);
    }

    let starting_point = Point {
        x: 500 - min_x + x_increase_border_size,
        y: 0,
    };
    return (map, starting_point);
}

fn drop_sand(map: &mut Map, starting_point: &Point) -> bool {
    let mut current_sand_location = Point {
        x: starting_point.x,
        y: starting_point.y,
    };

    let mut end_point_reached = false;
    while !end_point_reached {
        if current_sand_location.y == map.len() - 1 {
            return false; // Reached bottom
        }
        if map[current_sand_location.y + 1][current_sand_location.x] == None {
            current_sand_location.y += 1;
        } else if current_sand_location.x > 0
            && map[current_sand_location.y + 1][current_sand_location.x - 1] == None
        {
            current_sand_location.y += 1;
            current_sand_location.x -= 1;
        } else if current_sand_location.x < map[0].len() - 1
            && map[current_sand_location.y + 1][current_sand_location.x + 1] == None
        {
            current_sand_location.y += 1;
            current_sand_location.x += 1;
        } else {
            if current_sand_location.x == starting_point.x
                && current_sand_location.y == starting_point.y
            {
                return false;
            }
            end_point_reached = true;
        }
    }

    // Update location of new sand
    map[current_sand_location.y][current_sand_location.x] = Some(Material::Sand);
    return true; // Returns that map has changed
}

fn nr_units_until_resting_state_reached(map: &Map, starting_point: Point) -> usize {
    // println!(
    //     "Starting point: {:?},{:?}",
    //     starting_point.x, starting_point.y
    // );
    // println!("Map:");
    // print_map(&map);

    let mut current_map_state = map.to_owned();
    let mut count = 0;
    while drop_sand(&mut current_map_state, &starting_point) {
        // print_map(&current_map_state);
        count += 1;
    }

    println!("Map:");
    print_map(&current_map_state);
    return count + 1; // We did not actually place the last one
}

fn main() {
    let (map, starting_point) = parse_input("assets/input.in");
    println!(
        "Solution: {:?}",
        &nr_units_until_resting_state_reached(&map, starting_point)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_example() {
        let (map, starting_point) = parse_input("assets/example.in");
        assert_eq!(
            93,
            nr_units_until_resting_state_reached(&map, starting_point)
        );
    }
}
