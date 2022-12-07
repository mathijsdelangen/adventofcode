use std::collections::HashMap;
use std::fs;

const DELIMETER: &str = "/";
const SIZE_CHECK: usize = 100000;

type DirectoryInfo = HashMap<String, Directory>;
type FileInfo = HashMap<String, File>;

#[derive(Debug, Default)]
struct Directory {
    directories: DirectoryInfo,
    files: FileInfo,
    size: usize,
}

impl Directory {
    fn add_file_by_path(&mut self, path: &str, file: &File) {
        self.size += file.size;
        if path.contains(DELIMETER) {
            let parts = path.split_once(DELIMETER).unwrap();
            let directory_name = parts.0;
            let new_path = parts.1;
            if !self.directories.contains_key(directory_name) {
                self.directories
                    .insert(directory_name.to_string(), Directory::default());
            }
            let directories = self.directories.get_mut(directory_name).unwrap();
            directories.add_file_by_path(new_path, file);
        } else {
            self.files.insert(path.to_owned(), *file);
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct File {
    size: usize,
}

fn command_goto_dir(dir: &String, current_path: &mut String) -> () {
    println!("Move to {:?}", dir);
    match dir.as_str() {
        ".." => {
            let delim_removed = String::from(current_path.rsplit_once(DELIMETER).unwrap().0);
            let new_path = delim_removed.rsplit_once(DELIMETER).unwrap().0;
            *current_path = String::from(new_path);
        }
        "/" => *current_path = String::from(""), // Only used at start and we skip this
        _ => current_path.push_str(dir),
    }
    current_path.push_str(DELIMETER);
    println!("Current path {:?}", current_path);
}

fn parse_file_list(files: Vec<&str>, file_system: &mut FileInfo, current_path: &mut String) -> () {
    println!("Parse file list: {:?}", files);
    for f in files {
        if f.starts_with("dir") {
            // Don't care
        } else {
            let file_properties: Vec<&str> = f.split(" ").collect();
            let mut file_path = current_path.to_owned();
            file_path.push_str(file_properties[1]);

            file_system.insert(
                file_path.to_string(),
                File {
                    size: file_properties[0].parse::<usize>().unwrap(),
                },
            );
        }
    }
    println!("current directory list: {:?}", file_system);
}

fn parse_command(commands: &str, file_system: &mut FileInfo, current_path: &mut String) -> () {
    let lines: Vec<&str> = commands.split("\n").map(|l| l.trim()).collect();
    // First line is command
    let command = *lines.first().unwrap();
    if command.starts_with("cd") {
        command_goto_dir(&command.replace("cd ", ""), current_path)
    } else if command.starts_with("ls") {
        parse_file_list(lines[1..].to_vec(), file_system, current_path)
    } else {
        panic!("I do not know what to do with {:?}!", command)
    }
}

fn parse_input(input_file: &str) -> FileInfo {
    // This is the root directory
    let mut file_system = FileInfo::default();
    let current_path: &mut String = &mut String::from("/");

    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let commands: Vec<&str> = input.split("\n$ ").map(|l| l.trim()).collect();
    for command in &commands[1..] {
        parse_command(command, &mut file_system, current_path);
    }

    return file_system;
}

fn sum_of_sizes(directory: &Directory) -> usize {
    println!("My size: {:?}", directory.size);
    let mut solution = 0;
    if directory.size <= SIZE_CHECK {
        solution += directory.size;
    }
    for d in directory.directories.values() {
        solution += sum_of_sizes(d)
    }

    return solution;
}

fn first_solution(input_file: &str) -> usize {
    let flat_files = parse_input(input_file);
    let file_system = files_to_filesystem(&flat_files);
    println!("File system: {:?}", file_system);
    return sum_of_sizes(&file_system);
}

fn files_to_filesystem(files: &FileInfo) -> Directory {
    let mut file_system = Directory::default();
    for f in files {
        file_system.add_file_by_path(f.0, f.1);
    }
    return file_system;
}

fn main() {
    println!("Solution 1: {:?}", first_solution(&"assets/input.in"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_example_1() {
        assert_eq!(95437, first_solution("assets/example.in"));
    }

    #[test]
    fn validate_example_2() {
        //assert_eq!(TBD, solution(&parse_input("assets/example.in")));
    }
}
