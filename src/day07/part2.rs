use crate::day07::{Input, Output};

pub fn solve(input: &Input) -> Output {
    const MAX_STORAGE: usize = 70000000;
    const NEEDED_SPACE: usize = 30000000;
    const MAX_ABLE_TO_USE_SPACE: usize = MAX_STORAGE - NEEDED_SPACE;

    let used_space: usize = input[0].size;
    let minimum_space_needed: usize = used_space - MAX_ABLE_TO_USE_SPACE; // 10822529
    
    let mut smallest_possible_dir: usize = MAX_STORAGE;

    for dir in input {
        let dir_size = dir.size;

        // Is current directory smaller than current potential result?
        if dir_size < smallest_possible_dir {
            // Is it larger or equal to the minimum space needed?
            if dir_size >= minimum_space_needed {
                smallest_possible_dir = dir_size;
            }
        }
    };

    (smallest_possible_dir as u128).into()
}
