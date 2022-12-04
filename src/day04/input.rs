use crate::day04::Input;

const EXAMPLE_INPUT: &str = include_str!("../../input/04/example.txt");
const INPUT: &str = include_str!("../../input/04/input.txt");

pub fn read() -> Input {
    read_internal(INPUT)
}

pub fn read_example() -> Input {
    read_internal(EXAMPLE_INPUT)
}

pub fn read_internal(input: &str) -> Input {
    input
        .split("\n") 
        .map(get_pair_ranges) 
        .collect()
}

pub fn get_pair_ranges(pair: &str) -> Vec<Vec<usize>> {
    pair
        .split(",") 
        .map(|range| parse_string_pair_to_num(&mut range.split("-").collect())) // [['71', '71'], ['42', '72']]
        .collect() 
}

pub fn parse_string_pair_to_num(str_pair: &mut Vec<&str>) -> Vec<usize> {
    (str_pair.iter().map(|s| s.parse().unwrap()).collect())
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
