use crate::day07::{Input, Output};

pub fn solve(input: &Input) -> Output {
    const MAX: usize = 100000;
    input.iter().map(|dir| {
        if dir.size <= MAX {
            return dir.size as u128;
        } else {
            return 0 as u128;
        }
    }).sum::<u128>().into()
}
