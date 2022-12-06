use crate::day05::Input;

const EXAMPLE_INPUT: &str = include_str!("../../input/05/example.txt");
const INPUT: &str = include_str!("../../input/05/input.txt");

pub fn read() -> Input {
    read_internal(INPUT)
}

pub fn read_example() -> Input {
    read_internal(EXAMPLE_INPUT)
}

pub fn read_internal(input: &str) -> Input {
    let stacks_and_instructions: Vec<&str> = input.split("\n\n").collect();
    let mut stacks = stacks_and_instructions[0];
    let mut instructions = stacks_and_instructions[1];
    let stacks = format_stacks(stacks);
    let instructions = format_instructions(instructions);
    (stacks, instructions).into()
}

pub fn format_stacks(rows: &str) -> Vec<Vec<char>> {
    let mut rows: Vec<&str> = rows.split("\n").collect();
    rows.pop();
    let row_length = rows[0].len();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); (row_length + 1) / 4];
    for row in rows {
        let row: Vec<char> = row.chars().collect();
        let mut index = 0;
        for n in (1..row_length + 1).step_by(4) {
            if !row[n].is_whitespace() {
                stacks[index].push(row[n]);
            }
            index += 1;
        }
    }
   
    // stacks.iter_mut().for_each(|s| s.reverse());
    (stacks)
}

pub fn format_instructions(rows: &str) -> Vec<[usize; 3]> {
    let mut instructions: Vec<[usize; 3]> = Vec::new();
    let rows: Vec<&str> = rows.split("\n").collect();
    for row in rows {
        let row: Vec<&str> = row.split(" ").collect();
        let row: [usize; 3] = [row[1].parse().unwrap(), row[3].parse().unwrap(), row[5].parse().unwrap()];
        instructions.push(row);
    }
    (instructions)
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
