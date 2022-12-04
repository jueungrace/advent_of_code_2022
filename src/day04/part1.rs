use crate::day04::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut fully_contained_pairs_count = 0;

    for pair in input {
        let mut first_range: &Vec<usize> = &pair[0];
        let mut second_range = &pair[1];

        // TODO: find a better way of determining which range should come first
        if is_fully_contained(first_range, second_range) || is_fully_contained(second_range, first_range) {
            fully_contained_pairs_count += 1;
        }
    }

    fully_contained_pairs_count.into()
}

pub fn is_fully_contained(first_range: &Vec<usize>, second_range: &Vec<usize>) -> bool {
    let s1 = first_range[0];
    let e1 = first_range[1];

    let s2 = second_range[0];
    let e2 = second_range[1];

    (s2 >= s1 && e2 <= e1)
}
