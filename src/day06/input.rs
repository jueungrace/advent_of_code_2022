use crate::day06::Input;

const EXAMPLE_INPUT: &str = include_str!("../../input/06/example.txt");
const INPUT: &str = include_str!("../../input/06/input.txt");

pub fn read() -> Input {
    read_internal(INPUT)
}

pub fn read_example() -> Input {
    read_internal(EXAMPLE_INPUT)
}

pub fn read_internal(input: &str) -> Input {
    input.trim().chars().collect()
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
