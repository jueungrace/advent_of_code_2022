use crate::day06::{Input, Output};
use itertools::Itertools;

const MESSAGE_SIZE: usize = 14;

pub fn solve(input: &Input) -> Output {
    input.windows(MESSAGE_SIZE)
        .enumerate()
        .filter(|(i, window)| window.into_iter().all_unique())
        .map(|(i, window)| (i + MESSAGE_SIZE) as u128)
        .next()
        .unwrap_or_default()
        .into()
}
