use std::collections::HashMap;
use crate::day03::{Input, Output, get_priority};

// REFACTOR: after input only shows chars
pub fn solve(input: &Input) -> Output {
    let mut sum_of_priorities: u128 = 0;
    let mut found = false;

    for (i, (first, second)) in input.into_iter().enumerate().step_by(3) {

        let next_sack = input.get(i + 1).unwrap();
        let next_next_sack = input.get(i + 2).unwrap();

        for (c, _) in first {
            if check_both_compartments(&next_sack.0, &next_sack.1, *c) && check_both_compartments(&next_next_sack.0, &next_next_sack.1, *c) {
                sum_of_priorities += get_priority(*c);
                found = true;
            }
        }
        
        if found {
            found = false;
            continue;
        } else {
            for (c, _) in second {
                if check_both_compartments(&next_sack.0, &next_sack.1, *c) && check_both_compartments(&next_next_sack.0, &next_next_sack.1, *c) {
                    sum_of_priorities += get_priority(*c);
                }
            }
        }
    
    }

    sum_of_priorities.into()
}

// Checks if Character is present in either compartment.
pub fn check_both_compartments(first: &HashMap<char, usize>, second: &HashMap<char, usize>, c: char) -> bool {
    first.contains_key(&c) || second.contains_key(&c)
}