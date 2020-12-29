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

fn get_tile_by_id(id: u32, tiles: Vec<puzzle::Tile>) -> puzzle::Tile {
    for tile in tiles { if tile.id == id { return tile}}
    panic!("No tile found")
}

fn get_tile(id: u32, tiles: Vec<&puzzle::Tile>) -> &puzzle::Tile {
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

fn print_puzzle_pieces(puzzle_positions: &[[u32;12];12], tiles: &Vec<puzzle::Tile>) {
    for a in puzzle_positions {
        for line in 0..10 {
            for b in a { 
                if *b == 0 {continue}
                let tile = get_tile_by_id(*b, tiles.to_vec());
                
                print!("{}|", tile.tile_description[line]);
            }
            println!();
        }
        println!("----------");
    }
}

fn print_puzzle_tiles(puzzle_tiles : &Vec<Vec<puzzle::Tile>>){
    for line in puzzle_tiles {
        if line.len() > 0
        {
            for idx in 0..line[0].tile_description.len() {
                for tile in line {
                    print!("{}|", tile.tile_description[idx]);
                }
                println!();
            }
            println!("----------");
        }
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

            let neighbors_1 = find_neighbors(&get_tile_by_id(puzzle_solution[i][iteration-1], tiles.to_vec()),&tiles);
            let mut allowed_ids = Vec::new();
            if i > 0 {
                let neighbors_2 = find_neighbors(&get_tile_by_id(puzzle_solution[i-1][iteration], tiles.to_vec()),&tiles);
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
            let neighbors_1 = find_neighbors(&get_tile_by_id(puzzle_solution[iteration-1][i], tiles.to_vec()),&tiles);
            let mut allowed_ids = Vec::new();
            if i > 0 {
                let neighbors_2 = find_neighbors(&get_tile_by_id(puzzle_solution[iteration][i-1], tiles.to_vec()),&tiles);
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
            let neighbors_1 = find_neighbors(&get_tile_by_id(puzzle_solution[iteration-1][iteration], tiles.to_vec()),&tiles);
            let neighbors_2 = find_neighbors(&get_tile_by_id(puzzle_solution[iteration][iteration-1], tiles.to_vec()),&tiles);
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

fn nr_rotations_needed(request_border: usize, current_border: usize) -> isize {
    return ((current_border as isize - request_border as isize) + 4) % 4;
}

fn solve_puzzle_tiles(puzzle_positions: &[[u32;12];12], tile_list: Vec<puzzle::Tile>) -> Vec<Vec<puzzle::Tile>> {
    let mut solved_tiles : Vec<Vec<puzzle::Tile>> = Vec::new();

    for j in 0..puzzle_positions.len() {
        let mut solved_tiles_row = Vec::new();
        for i in 0..puzzle_positions[j].len() { 
            if puzzle_positions[i][j] == 0 {continue}

            let pos_1 = puzzle_positions[i][j];            
            // Special case, most left side -> Check pieze above
            if i == 0 {
                // Extra special case, starting piece
                if j == 0 {
                    let pos_2 = puzzle_positions[i+1][j];

                    let mut tile = get_tile_by_id(pos_1, tile_list.to_vec());
                    let neighbor = get_tile_by_id(pos_2, tile_list.to_vec());
                    let neighbor_below = get_tile_by_id(puzzle_positions[i][j+1], tile_list.to_vec());

                    // Find common border with piece right of this one
                    let common_border = puzzle::Tile::find_common_border(&tile, &neighbor);
                    println!("Common border between pos1 and right is ({},{})", common_border.0, common_border.1);
                    println!("Tile should rotate {} (X -> 0) times", nr_rotations_needed(common_border.0, 0));
                    for _ in 0..nr_rotations_needed(common_border.0, 0) { println!("Rotate");tile.rotate() }
                    //println!("Neighbor should rotate {} (X -> 2) times", nr_rotations_needed(common_border.1, 2));
                    //for _ in 0..nr_rotations_needed(common_border.1, 2) { neighbor.rotate() }

                    // Find common border with piece below this one
                    let common_border = puzzle::Tile::find_common_border(&tile, &neighbor_below);
                    println!("Common border between pos1 and below is ({},{})", common_border.0, common_border.1);
                    if common_border.0 != 1 { println!("Flip");tile.flip() }
                    println!("Add tile");
                    solved_tiles_row.push(tile);
                }
                else
                {
                    // Check pieze above                   
                    let mut tile = get_tile_by_id(pos_1, tile_list.to_vec());
                    let neighbor_above = solved_tiles.last().unwrap()[0].to_owned();

                    let common_border = puzzle::Tile::find_common_border(&neighbor_above, &tile);
                    println!("Common border between neighbor above and tile is ({},{})", common_border.0, common_border.1);
                    println!("Tile should rotate {} (X -> 3) times", nr_rotations_needed(common_border.1, 3));
                    for _ in 0..nr_rotations_needed(common_border.1, 3) { println!("Rotate");tile.rotate() }
                    if tile.get_borders()[3] != neighbor_above.get_borders()[1] { println!("FlipAndRotate");tile.flip_and_rotate_back() }
                    println!("Add tile");
                    solved_tiles_row.push(tile);
                }

            }
            else
            {
                // Check piece to the left               
                let mut tile = get_tile_by_id(pos_1, tile_list.to_vec());
                let left_neighbor = solved_tiles_row.last().unwrap().to_owned();

                let common_border = puzzle::Tile::find_common_border(&left_neighbor, &tile);
                println!("Common border between left_neighbor and tile is ({},{})", common_border.0, common_border.1);
                println!("Tile should rotate {} (X -> 2) times", nr_rotations_needed(common_border.1, 2));
                for _ in 0..nr_rotations_needed(common_border.1, 2) { println!("Rotate");tile.rotate() }
                if tile.get_borders()[2] != left_neighbor.get_borders()[0] { println!("Flip");tile.flip() }
                println!("Add tile");
                solved_tiles_row.push(tile);
            }
        }
        solved_tiles.push(solved_tiles_row);
    }
    
    return solved_tiles;
}

fn print_solved_puzzle(tiles: &Vec<Vec<puzzle::Tile>>) {
    for line_with_tiles in tiles {
        if line_with_tiles.len() > 0 {
            for idx in 1..line_with_tiles[0].tile_description.len()-1 {
                let mut puzzle_line : Vec<String>;
                for tile in line_with_tiles {
                    print!("{}",tile.tile_description[idx][1..tile.tile_description[idx].len()-1].to_string());
                }
                println!();
            }
        }
    }
}

/*
01234567890123456789
                  # 
#    ##    ##    ###
 #  #  #  #  #  #   
*/
fn has_seamonster_pattern(line1: &str, line2: &str, line3: &str) -> bool {
    return line1.chars().nth(18).unwrap() == '#' &&
    line2.chars().nth(0).unwrap() == '#' &&
    line2.chars().nth(5).unwrap() == '#' &&
    line2.chars().nth(6).unwrap() == '#' &&
    line2.chars().nth(11).unwrap() == '#' &&
    line2.chars().nth(12).unwrap() == '#' &&
    line2.chars().nth(17).unwrap() == '#' &&
    line2.chars().nth(18).unwrap() == '#' &&
    line2.chars().nth(19).unwrap() == '#' &&
    line3.chars().nth(1).unwrap() == '#' &&
    line3.chars().nth(4).unwrap() == '#' &&
    line3.chars().nth(7).unwrap() == '#' &&
    line3.chars().nth(10).unwrap() == '#' &&
    line3.chars().nth(13).unwrap() == '#' &&
    line3.chars().nth(16).unwrap() == '#';
}

fn image_contains_seamonters(image: &Vec<String>) -> bool {
    for idx in 0..image.len()-2 {
        //println!("Checking image @ idx={}", idx);
        // Find sea monster pattern
        for loc in 0..image[idx].len()-19 {
            //println!("Checking image @loc={}", loc);
            if has_seamonster_pattern(&image[idx][loc..loc+20], &image[idx+1][loc..loc+20], &image[idx+2][loc..loc+20]) { println!("Found seamonster!"); return true;}
        }
    }
    return false;
}

fn flip_image(image: &Vec<String>) -> Vec<String> {
    let mut new_image = image.to_owned();
    new_image.reverse();
    return new_image;
}

fn rotate_image(image: &Vec<String>) -> Vec<String> {
    let mut current_image = image.to_owned();
    let mut new_image : Vec<String> = vec!["".to_string();image.len()];
    
    current_image.reverse();
    for (_,line) in current_image.iter().enumerate() {
        for (c_idx,c) in line.chars().enumerate() {
            new_image[c_idx] += &c.to_string();
        }
    }
    return new_image;
}

fn main() {   
    /*
    let input_file = "assets/dec_20.in";
    let tiles = parse_input(input_file);
    println!("Solution: {}", determine_corners(&tiles) );

    // Solution 2
    let puzzle_positions = solve_puzzle_positions(&tiles);
    print_puzzle(&puzzle_positions);
    print_puzzle_pieces(&puzzle_positions, &tiles);

    let solved_tiles = solve_puzzle_tiles(&puzzle_positions, tiles);
    println!("Solved tiles length: {}", solved_tiles.len());
    print_puzzle_tiles(&solved_tiles);
    print_solved_puzzle(&solved_tiles);
    */

    let solved_image_input = fs::read_to_string("assets/puzzle_output").expect("Something went wrong reading the file");
    let solved_image : Vec<&str> = solved_image_input.split("\r\n").collect();

    let image : Vec<String> = solved_image.iter().map(|&l| l.to_string()).collect();
    image_contains_seamonters(&image);
    image_contains_seamonters(&rotate_image(&image));
    image_contains_seamonters(&rotate_image(&rotate_image(&image)));
    image_contains_seamonters(&rotate_image(&rotate_image(&rotate_image(&image))));
    image_contains_seamonters(&rotate_image(&rotate_image(&rotate_image(&rotate_image(&image)))));

    let flipped_image = flip_image(&image);
    image_contains_seamonters(&flipped_image);
    image_contains_seamonters(&rotate_image(&flipped_image));
    image_contains_seamonters(&rotate_image(&rotate_image(&flipped_image)));
    image_contains_seamonters(&rotate_image(&rotate_image(&rotate_image(&flipped_image))));
    image_contains_seamonters(&rotate_image(&rotate_image(&rotate_image(&rotate_image(&flipped_image)))));

    let flipped_rotated_image = flip_image(&rotate_image(&image));
    image_contains_seamonters(&flipped_rotated_image);
    image_contains_seamonters(&rotate_image(&flipped_rotated_image));
    image_contains_seamonters(&rotate_image(&rotate_image(&flipped_rotated_image)));
    image_contains_seamonters(&rotate_image(&rotate_image(&rotate_image(&flipped_rotated_image))));
    image_contains_seamonters(&rotate_image(&rotate_image(&rotate_image(&rotate_image(&flipped_rotated_image)))));

    //let puzzle_lines : Vec<String> = puzzle.iter().map(|l| l.join("")).collect();
    //println!("Solved puzzle:\r\n{}", puzzle_lines.join("\r\n"));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn validate_example(){
        let input_file = "assets/dec_20_example.in";
        assert_eq!(20899048083289, determine_corners(&parse_input(input_file)));
    }

    #[test]
    fn validate_contains_seamonsters() {
        let solved_image = [
            "..................#.",
            "#....##....##....###",
            ".#..#..#..#..#..#...",
        ];
        let image : Vec<String> = solved_image.iter().map(|&l| l.to_string()).collect();

        assert_eq!(true, image_contains_seamonters(&image));

        let solved_image = [
            "##############...........#",
            "###..................#....",
            "####....##....##....###...",
            "###.#..#..#..#..#..#......",
            "##############...........#",
        ];

        let image : Vec<String> = solved_image.iter().map(|&l| l.to_string()).collect();

        assert_eq!(true, image_contains_seamonters(&image));
    }
}