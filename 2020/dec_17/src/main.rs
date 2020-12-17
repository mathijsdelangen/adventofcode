use std::fs;

mod cube;

fn simulate_cycle(orig: &cube::Cube) -> cube::Cube {
    let mut cube = cube::Cube {width: orig.width+2, height: orig.height+2, max_w: orig.max_w+2, occupancies: vec![]};
    let mut points_in_cube = Vec::new();

    for w in 0..cube.max_w {
        for z in 0..cube.height {
            for y in 0..cube.width {
                for x in 0..cube.width {
                    let p = cube::Point4 {x:x, y:y, z:z, w:w};
                    let active_neighors = cube::count_occupied_neighbors(&orig.occupancies, p);
                    if cube::is_present(&orig.occupancies, &p) {
                        if active_neighors == 2 || active_neighors == 3 {
                            points_in_cube.push(cube::Point4 {x: p.x+1, y: p.y+1, z: p.z+1, w:p.w+1});
                        }
                    } else {
                        if active_neighors == 3 {
                            points_in_cube.push(cube::Point4 {x: p.x+1, y: p.y+1, z: p.z+1, w:p.w+1});
                        }
                    }
                }
            }
        }
    }

    cube.occupancies = points_in_cube;
    
    return cube;
}

fn determine_active_cubes(mut cube: cube::Cube) -> usize {

    //cube.print();
    for _i in 0..6 {
        //println!("Iteration {}", i+1);
        cube = simulate_cycle(&cube);
        cube.print();
    }

    return cube.occupancies.len();
}

fn main() {
    let input_file = "assets/dec_17.in";
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let cube = cube::Cube::new_from_str(&input);

    println!("Solution: {}", determine_active_cubes(cube));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_example(){
        let input_file = "assets/dec_17_example.in";
        let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
        let cube = cube::Cube::new_from_str(&input);
        assert_eq!(848, determine_active_cubes(cube));
    }
}