use std::cmp::max;

use crate::day08::Input;

const EXAMPLE_INPUT: &str = include_str!("../../input/08/example.txt");
const INPUT: &str = include_str!("../../input/08/input.txt");

pub fn read() -> Input {
    read_internal(INPUT)
}

pub fn read_example() -> Input {
    read_internal(EXAMPLE_INPUT)
}

pub fn read_internal(input: &str) -> Input {
    let mut grid: Vec<Vec<Vec<isize>>> = Vec::new();

    for (index, line) in input.lines().enumerate() {
        let mut max_size: isize = 0;
        let mut row: Vec<Vec<isize>> = Vec::new();

        // From left to right:
        line.chars().for_each(|s| {
            let mut tree: Vec<isize> = Vec::new(); // [ size or visibility, max from left, max from right, max from top, max from down ]
            let tree_size = s.to_digit(10).unwrap();

            // Add tree size to array:
            tree.push(tree_size as isize);

            // Insert max from left into tree array
            tree.push(max_size);
            
            // Recalculate max size from left so far:
            max_size = max(max_size, tree_size as isize);

            // Insert into row
            row.push(tree);
        });

        max_size = 0;

        // From right to left:
        for (i, tree) in row.iter_mut().rev().enumerate() {
            let tree_size = tree[0];
            tree.push(max_size);
            max_size = max(max_size, tree_size);
        }

        // Insert row into grid ([ size or visible, left_max, right_max ]):
        grid.push(row);
    };

    let columns: usize = grid[0].len();
    let rows: usize = grid.len();

    // Calculate max from top to bottom:
    for col in (0..columns) {
        let mut max_from_top: isize = 0;
        for row in (0..rows) {
            let tree_size = grid[row][col][0];
            grid[row][col].push(max_from_top);
            max_from_top = max(max_from_top, tree_size);
        }
    }

    // Calculate max from bottom to top:
    for col in (0..columns) {
        let mut max_from_btm: isize = 0;
        for row in (0..rows).rev() {
            let tree_size = grid[row][col][0];
            grid[row][col].push(max_from_btm);
            max_from_btm = max(max_from_btm, tree_size);
        }
    }

    (grid)
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