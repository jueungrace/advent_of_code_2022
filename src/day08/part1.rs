use crate::day08::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut visible_tree_count = 0;

    let num_rows = input.len();
    let num_cols = input[0].len();

    for (r, row) in input.iter().enumerate() {
        for (c, tree) in row.iter().enumerate() {
            if r == 0 || r == num_rows - 1 || c == 0 || c == num_cols - 1 {
                visible_tree_count += 1;
            } else if tree[0] > *tree.iter().min().unwrap() {
                visible_tree_count += 1;
            }
        }
    };

    (visible_tree_count.into())
}
