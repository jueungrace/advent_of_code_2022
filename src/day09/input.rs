use crate::day09::Input;

use super::Direction;

const EXAMPLE_INPUT: &str = include_str!("../../input/09/example.txt");
const INPUT: &str = include_str!("../../input/09/input.txt");

pub fn read() -> Input {
    read_internal(INPUT)
}

pub fn read_example() -> Input {
    read_internal(EXAMPLE_INPUT)
}

pub fn read_internal(input: &str) -> Input {
    input.lines().map(|line| {
        let direction_and_steps: Vec<&str> = line.split_whitespace().collect();
        let direction: &str = direction_and_steps[0];
        let direction: Direction = match direction {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            &_ => panic!(),
        };
        (direction, direction_and_steps[1].parse().unwrap())
    }).collect()
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
