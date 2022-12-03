use std::collections::HashMap;

use crate::day03::Input;

const EXAMPLE_INPUT: &str = include_str!("../../input/03/example.txt");
const INPUT: &str = include_str!("../../input/03/input.txt");

pub fn read() -> Input {
    read_internal(INPUT)
}

pub fn read_example() -> Input {
    read_internal(EXAMPLE_INPUT)
}

/**
 * A list of every item in each rucksack.
 * Each item is identified by a lowercase or uppercase letter.
 * One rucksack's contents = a single string.
 * There are an equal amount of items in the two compartments.
 */

pub fn read_internal(input: &str) -> Input {
    input
        .trim()
        .split('\n')
        .map(get_inventory)
        .collect()
}

// REFACTOR: to only show chars not counts
// Do we actually need the item count or just the character?
pub fn get_inventory(items: &str) -> (HashMap<char, usize>, HashMap<char, usize>) {
    let mut first_compartment: HashMap<char, usize> = HashMap::new();
    let mut second_compartment: HashMap<char, usize> = HashMap::new();
    let start_of_second_compartment: usize = items.chars().count() / 2;

    for (i, c) in items.chars().enumerate() {
        if i < start_of_second_compartment {
            add_or_update_inventory(&mut first_compartment, c);
        } else {
            add_or_update_inventory(&mut second_compartment, c);
        }
    }

    (first_compartment, second_compartment)
}

pub fn add_or_update_inventory(inventory: &mut HashMap<char, usize>, c: char) {
    if inventory.contains_key(&c) {
        *inventory.get_mut(&c).unwrap() += 1;
    } else {
        inventory.insert(c, 1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input() {
        let example_input = read_example();
        println!("{:?}", example_input);
    }
}
