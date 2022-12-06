use crate::day05::{Input, Output};

pub fn solve(input: Input) -> Output {
    let mut result: String = "".to_string();
    let instructions = input.1;
    let mut stacks = input.0;

    println!("{:?}", stacks);

    for instruction in instructions {
        let from = instruction[1] - 1;
        let to = instruction[2] - 1;
        let amount = instruction[0];

        let mut need_to_move: Vec<char> = stacks[from].drain(0..amount).rev().collect();
        stacks[to] = [need_to_move.as_slice(), stacks[to].as_slice()].concat();
    }

    stacks.iter().for_each(|stack| result.push_str(&stack[0].to_string()));

    (result.to_string().into())
}
