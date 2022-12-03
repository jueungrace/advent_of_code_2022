use std::collections::HashMap;
use crate::day03::{Input, Output, get_priority};

// REFACTOR: after input only shows chars
pub fn solve(input: &Input) -> Output {
    let mut sum_of_priorities: u128 = 0;

    for inventory in input {
        for (c, _) in &inventory.0 {
            if inventory.1.contains_key(&c) {
                sum_of_priorities += get_priority(*c);
                break;
            }
        }
    }

    sum_of_priorities.into()
}