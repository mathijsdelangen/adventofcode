use std::fs;
mod puzzle;

fn parse_input(input_file: &str) -> Vec<puzzle::Tile> {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let tile_descriptions : Vec<&str> = input.split("\r\n\r\n").collect();
    return tile_descriptions.iter().map(|d| puzzle::Tile::from_str(d)).collect();
}

fn determine_corners(tiles: &Vec<puzzle::Tile>) -> u128 {
    let mut result = 1u128;
    for tile_a in tiles {
        let mut num_tiles_matching_border = 0;
        for tile_b in tiles {
            if tile_a.id == tile_b.id {continue;}
            if puzzle::Tile::matches_border(tile_a, tile_b) { num_tiles_matching_border += 1; }
        }
        if num_tiles_matching_border == 2 { result *= tile_a.id as u128; }
    }
    return result;
}

fn find_corner(tiles: &Vec<puzzle::Tile>) -> u32 {
    for tile_a in tiles {
        let mut num_tiles_matching_border = 0;
        for tile_b in tiles {
            if tile_a.id == tile_b.id {continue;}
            if puzzle::Tile::matches_border(tile_a, tile_b) { num_tiles_matching_border += 1; }
        }
        if num_tiles_matching_border == 2 {
            return tile_a.id;
        }
    }
    return 0;
}

fn find_neighbors(tile: &puzzle::Tile, tiles: &Vec<puzzle::Tile>) -> Vec<u32> {
    let mut neighbors = Vec::new();
    for tile_a in tiles {
        if tile_a.id == tile.id {continue;}
        if puzzle::Tile::matches_border(tile_a, tile) { neighbors.push(tile_a.id) }
    }
    
    return neighbors;
}

fn get_tile_by_id(id: u32, tiles: &Vec<puzzle::Tile>) -> &puzzle::Tile {
    for tile in tiles { if tile.id == id { return tile}}
    panic!("No tile found")
}

fn print_puzzle(puzzle_solution: &[[u32;12];12]) {
    for a in puzzle_solution {
        for b in a { 
            print!("{}\t",b);
        }
        println!("");
    }
}

fn print_puzzle_pieces(puzzle_solution: &[[u32;12];12], tiles: &Vec<puzzle::Tile>) {
    for a in puzzle_solution {
        for line in 0..10 {
            for b in a { 
                if *b == 0 {continue}
                let tile = get_tile_by_id(*b, &tiles);
                
                print!("{}|", tile.tile_description[line]);
            }
            println!();
        }
        println!("----------");
    }
}

fn solve_puzzle_positions(tiles: &Vec<puzzle::Tile>) -> [[u32;12];12] {

    let mut puzzle_solution = [[0;12];12];
    let mut puzzle_ids_left: Vec<u32> = tiles.iter().map(|t| t.id).collect();

    // Find first corner
    let corner = find_corner(&tiles);
    puzzle_solution[0][0] = corner;
    let index = puzzle_ids_left.iter().position(|x| *x == corner).unwrap();
    puzzle_ids_left.remove(index);

    let mut iteration = 0;
    print_puzzle(&puzzle_solution);
    while puzzle_ids_left.len() > 0 {
        iteration += 1;
        
        for i in 0..iteration {
            print_puzzle(&puzzle_solution);
            println!("{} ids left", puzzle_ids_left.len());
            println!("Find tile id at position ({},{}) for ID {}", i, iteration-1, puzzle_solution[i][iteration-1]);

            let neighbors_1 = find_neighbors(get_tile_by_id(puzzle_solution[i][iteration-1], &tiles),&tiles);
            let mut allowed_ids = Vec::new();
            if i > 0 {
                let neighbors_2 = find_neighbors(get_tile_by_id(puzzle_solution[i-1][iteration], &tiles),&tiles);
                for neighbor in neighbors_2 {
                    if neighbors_1.contains(&neighbor) {
                        allowed_ids.push(neighbor);
                    }
                }
            }
            else {
                allowed_ids = neighbors_1;
            }

            let mut puzzle_piece_id = 0;
            for neighbor in allowed_ids {
                if puzzle_ids_left.contains(&neighbor) {println!(" {} is a allowed neighbor", neighbor);puzzle_piece_id = neighbor} else {println!(" {} is a neighbor, but not allowed", neighbor);}
            }

            puzzle_solution[i][iteration] = puzzle_piece_id;
            let index = puzzle_ids_left.iter().position(|x| *x == puzzle_piece_id).unwrap();
            puzzle_ids_left.remove(index);
            
        }

        for i in 0..iteration {
            print_puzzle(&puzzle_solution);
            let neighbors_1 = find_neighbors(get_tile_by_id(puzzle_solution[iteration-1][i], &tiles),&tiles);
            let mut allowed_ids = Vec::new();
            if i > 0 {
                let neighbors_2 = find_neighbors(get_tile_by_id(puzzle_solution[iteration][i-1], &tiles),&tiles);
                for neighbor in neighbors_2 {
                    if neighbors_1.contains(&neighbor) {
                        allowed_ids.push(neighbor);
                    }
                }
            }
            else {
                allowed_ids = neighbors_1;
            }

            let mut puzzle_piece_id = 0;
            for neighbor in allowed_ids {
                if puzzle_ids_left.contains(&neighbor) {println!(" {} is a allowed neighbor", neighbor);puzzle_piece_id = neighbor} else {println!(" {} is a neighbor, but not allowed", neighbor);}
            }
            
            puzzle_solution[iteration][i] = puzzle_piece_id;
            let index = puzzle_ids_left.iter().position(|x| *x == puzzle_piece_id).unwrap();
            puzzle_ids_left.remove(index);
        }

        {
            let neighbors_1 = find_neighbors(get_tile_by_id(puzzle_solution[iteration-1][iteration], &tiles),&tiles);
            let neighbors_2 = find_neighbors(get_tile_by_id(puzzle_solution[iteration][iteration-1], &tiles),&tiles);
            let mut puzzle_piece_id = 0;
            for neighbor in neighbors_1 {
                if puzzle_ids_left.contains(&neighbor) && neighbors_2.contains(&neighbor) {puzzle_piece_id = neighbor}
            }
            
            puzzle_solution[iteration][iteration] = puzzle_piece_id;
            let index = puzzle_ids_left.iter().position(|x| *x == puzzle_piece_id).unwrap();
            puzzle_ids_left.remove(index);
        }
        print_puzzle(&puzzle_solution);
    }

    return puzzle_solution;
}

fn determine_puzzle_output(puzzle_positions: &[[u32;12];12], tiles: &Vec<puzzle::Tile>) -> Vec<String> {

    for i in 0..puzzle_positions.len()-1 {
        for j in 0..puzzle_positions[i].len()-1 { 
            if puzzle_positions[i][j+1] == 0 {continue}
            
            let pos_1 = puzzle_positions[i][j];
            let pos_2 = puzzle_positions[i][j+1];
            
            let tile_1 = get_tile_by_id(pos_1, &tiles);
            let tile_2 = get_tile_by_id(pos_2, &tiles);

            let (nr_rot_1, nr_rot_2) = puzzle::Tile::find_common_border(tile_1,tile_2);

        }
    }


    return vec![String::from("");10];
}

fn main() {
    //let input_file = "assets/dec_20.in";
    let input_file = "assets/dec_20_example.in";
    let tiles = parse_input(input_file);
    println!("Solution: {}", determine_corners(&tiles) );

    let puzzle = solve_puzzle_positions(&tiles);
    print_puzzle(&puzzle);
    print_puzzle_pieces(&puzzle, &tiles);
    
    let solved_puzzle = determine_puzzle_output(&puzzle, &tiles);
    println!("Puzzle IDs:");

    
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_example(){
        let input_file = "assets/dec_20_example.in";
        assert_eq!(20899048083289, determine_corners(&parse_input(input_file)));
    }
}