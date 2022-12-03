use std::collections::HashMap;
use crate::day03::{Input, Output};

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

pub fn get_priority(c: char) -> u128 {
    let is_uppercase: bool = c.is_uppercase();
    
    let p = match c.to_ascii_lowercase() {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        _ => 0
    };

    if is_uppercase {
        return p + 26;
    } else {
        return p;
    }

}