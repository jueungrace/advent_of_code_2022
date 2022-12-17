use std::{collections::HashMap, iter::Skip};

use crate::day07::{Input, Content};

// This is basically @gbegerow's solution because I gave up after struggling with Rust types.
// https://github.com/gbegerow/advent-of-code/blob/main/aoc_2022_07/src/lib.rs

const EXAMPLE_INPUT: &str = include_str!("../../input/07/example.txt");
const INPUT: &str = include_str!("../../input/07/input.txt");

pub fn read() -> Input {
    read_internal(INPUT)
}

pub fn read_example() -> Input {
    read_internal(EXAMPLE_INPUT)
}

// Stores folders and their contents - no size calculations:
pub fn read_internal(input: &str) -> Input {
    
    let mut file_system = Vec::new();
    file_system.push(Content {
        name: String::from("/"),
        size: 0,
        parent: 0
    });

    let mut cwd: usize = 0;

    for mut line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();
        // You can either get a directory ("cd ______", "cd ..", "dir ______")
        if line[1] == "cd" {
            // cd .. or [dir_name]
            match line[2] {
                "/" => cwd = 0,
                ".." => cwd = file_system[cwd].parent,
                _ => {
                    file_system.push(Content {
                        name: line[1].to_string(),
                        size: 0,
                        parent: cwd
                    });
                    cwd = file_system.len() - 1;
                },
            }
        } else if line[0].chars().all(|c| c.is_numeric()) {
            let file_size: usize = line[0].parse().unwrap();
            // Go up the "file path" and increment size
            let mut pointer = cwd;
            loop {
                file_system[pointer].size += file_size;
                // Break out of the loop once we've hit the top of the directory
                if pointer == 0 {
                    break;
                }
                pointer = file_system[pointer].parent;
            }
        }
        
        // We don't care about "ls" or "dir ____" because we'll cd into every directory anyways.
    }

    (file_system)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input() {
        let example_input = read_example();
        println!("{:?}", example_input);
    }
}
