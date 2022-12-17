use std::collections::HashMap;
use itertools::Itertools;

use crate::day06::{Input, Output};

const WINDOW_SIZE: usize = 4;

pub fn solve(input: &Input) -> Output {
    input.windows(WINDOW_SIZE)
        .enumerate()
        .filter(|(i, window)| window.into_iter().all_unique())
        .map(|(i, window)| (i + WINDOW_SIZE) as u16)
        .next()
        .unwrap_or_default()
        .into()
}
