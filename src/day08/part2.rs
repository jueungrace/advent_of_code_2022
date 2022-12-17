use std::cmp::max;

use crate::day08::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut max_score: u32 = 0;
    let num_rows = input.len();
    let num_cols = input[0].len();

    // Avoid the edges - because at least one score is always 0.

    for r in (1..num_rows - 1) {
        for c in (1..num_cols - 1) {
            let tree = &input[r][c];

            // If tree size is the minimum of tree array, continue:
            if *tree.iter().min().unwrap() == tree[0] {
                continue;
            }

            let mut total_score: usize = 1;
            let mut score: usize = 0;
            let mut pointer = c as isize - 1;

            // Left
            while pointer >= 0 {
                // Break if hit same size:
                if input[r][pointer as usize][0] == tree[0] {
                    score += 1;
                    break;
                } // If hit max:
                else if input[r][pointer as usize][0] == tree[1] {
                    score += 1;
                    pointer -= 1;
                    // Check for consecutive maxes:
                    while pointer >= 0 && input[r][pointer as usize][0] == tree[1] {
                        score += 1;
                        pointer -= 1;
                    }
                } else {
                    score += 1;
                    pointer -= 1;
                }
            }

            if score == 0 {
                continue;
            }

            total_score *= score;
            score = 0;

            // Right
            pointer = c as isize + 1;
            while pointer < num_cols as isize {
                if input[r][pointer as usize][0] == tree[0] {
                    score += 1;
                    break;
                } // If hit max:
                else if input[r][pointer as usize][0] == tree[2] {
                    score += 1;
                    pointer += 1;
                    // Check for consecutive maxes:
                    while pointer < num_cols as isize && input[r][pointer as usize][0] == tree[2] {
                        score += 1;
                        pointer += 1;
                    }
                } else {
                    score += 1;
                    pointer +=1;
                }
                
            }

            if score == 0 {
                continue;
            }
            
            total_score *= score;
            score = 0;

            // Up
            pointer = r as isize - 1;
            while pointer >= 0 {
                if input[pointer as usize][c][0] == tree[0] {
                    score += 1;
                    break;
                } // If hit max:
                else if input[pointer as usize][c][0] == tree[3] {
                    score += 1;
                    pointer -= 1;
                    // Check for consecutive maxes:
                    while pointer >= 0 && input[pointer as usize][c][0] == tree[3] {
                        score += 1;
                        pointer -= 1;
                    }
                } else {
                    score += 1;
                    pointer -= 1;
                }
            } 

            if score == 0 {
                continue;
            }
            
            total_score *= score;
            score = 0;

            // Down
            pointer = r as isize + 1;
            while pointer < num_rows as isize {
                if input[pointer as usize][c][0] == tree[0] {
                    score += 1;
                    break;
                } // If hit max:
                else if input[pointer as usize][c][0] == tree[4] {
                    score += 1;
                    pointer += 1;
                    // Check for consecutive maxes:
                    while pointer < num_rows as isize && input[pointer as usize][c][0] == tree[4] {
                        score += 1;
                        pointer += 1;
                    }
                } else {
                    score += 1;
                    pointer += 1;
                }
                
            }

            total_score *= score;

            println!("{}", total_score);

            max_score = max(max_score, total_score as u32);
            
        }
    }

    (max_score.into())
}