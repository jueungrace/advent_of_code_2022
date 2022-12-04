use crate::day04::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut overlap_count = 0;

    for pair in input {
        let mut first_range: &Vec<usize> = &pair[0];
        let mut second_range = &pair[1];

        if (first_range[0] > second_range[0]) {
            first_range = &pair[1];
            second_range = &pair[0];
        }

        if does_overlap(first_range, second_range) {
            overlap_count += 1;
        }
    }

    overlap_count.into()
}

pub fn does_overlap(first_range: &Vec<usize>, second_range: &Vec<usize>) -> bool {
    (second_range[0] <= first_range[1])
}