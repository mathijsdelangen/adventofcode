use std::fs;

type Forest = Vec<Vec<Tree>>;
type TreeLine = Vec<Tree>;
type Tree = usize;

fn parse_line(line: &str) -> TreeLine {
    return line
        .chars()
        .map(|c| c.to_string().parse::<Tree>().unwrap())
        .collect::<Vec<_>>();
}

fn parse_input(input_file: &str) -> Forest {
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    return input
        .split("\n")
        .map(|l| parse_line(l.trim()))
        .collect::<Vec<_>>();
}

fn tree_visible(tree: Tree, other_trees: &Vec<Tree>) -> bool {   
    match other_trees.iter().max() {
        Some(val) => return tree > *val,
        None      => return true,
    }
}

fn visible_trees(forest: &Forest) -> usize {
    let mut count = 0;
    for y in 0..forest.len() {
        for x in 0..forest[0].len() {
            let tree = forest[y][x];
            let vertical_line: Vec<Tree> = forest.iter().map(|tree_line| tree_line[x]).collect();

            if tree_visible(tree, &forest[y][..x].to_vec())
                || tree_visible(tree, &forest[y][(x + 1)..].to_vec())
                || tree_visible(tree, &vertical_line[..y].to_vec())
                || tree_visible(tree, &vertical_line[(y+1)..].to_vec())
            {
                count += 1
            }
        }
    }

    return count;
}

fn main() {
    let forest = parse_input("assets/input.in");
    println!("Solution 1: {:?}", visible_trees(&forest));
    //println!("Solution 2: {:?}", visible_trees(&forest));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_tree_visible() {
        assert_eq!(true, tree_visible(3, &vec![0; 5]));
        assert_eq!(true, tree_visible(3, &vec![0; 0]));
        assert_eq!(false, tree_visible(3, &vec![4; 2]));
        assert_eq!(false, tree_visible(4, &vec![4; 2]));
    }

    #[test]
    fn validate_example_1() {
        assert_eq!(21, visible_trees(&parse_input("assets/example.in")));
    }

    #[test]
    fn validate_example_2() {
        //assert_eq!("MCD", solution(&parse_input("assets/example.in")));
    }
}
