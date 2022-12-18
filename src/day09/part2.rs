use std::collections::{HashMap, HashSet};

use crate::day09::{Input, Output, Direction};

use super::part1::{NextStep, are_touching};

const KNOTS: usize = 10;

pub fn solve(input: &Input) -> Output {
    // Holds coordinates of all knots:
    let mut knots: Vec<(i64, i64)> = vec![(0, 0); KNOTS];;

    // Holds coordinates visited by tail:
    let mut visited: HashMap<i64, HashSet<i64>> = HashMap::new();
    visited.insert(0, HashSet::from([0]));

    let mut unique_visited_positions = 1;

    input.iter().for_each(|(direction, mut num_steps)| {
        while num_steps > 0 {
            // Move the head:
            match direction {
                Direction::Down => knots[0].1 -= 1,
                Direction::Left => knots[0].0 -= 1,
                Direction::Right => knots[0].0 += 1,
                Direction::Up => knots[0].1 += 1,
                _ => panic!("Issue moving head position")
            };

            // Iterate through knots to check if any updates need to be made:
            check_knots(&mut knots, direction);

            // Check if current tail position has been visited:
            let tail = knots[KNOTS - 1];
            if visited.contains_key(&tail.0) {
                let mut tail_y_values = visited.get_mut(&tail.0).unwrap();
                // Tail y position not visited
                if !tail_y_values.contains(&tail.1) {
                    tail_y_values.insert(tail.1);
                    println!("{:?}", tail);
                    unique_visited_positions += 1;
                } 
            } else {
                println!("{:?}", tail);
                // X coordinate has never been visited
                visited.insert(tail.0, HashSet::from([tail.1]));
                unique_visited_positions += 1;
            }

            // println!("{:?}", visited);

            num_steps -= 1;
        }
    });

    (unique_visited_positions.into())
}

pub fn check_knots(knots: &mut Vec<(i64, i64)>, direction: &Direction) {
    let mut index = 0;
    let mut prev_direction: &Direction = direction;

    while index < KNOTS - 1 {
        // println!("{:?}, {:?}", knots[index], knots[index+1]);

        // Check if current pair needs to be updated:
        let next_step: NextStep = are_touching(knots[index], knots[index+1]);

        match next_step {
            NextStep::Diagonal => {
                // Current knot is north east:
                if knots[index].0 - knots[index+1].0 > 0 && knots[index].1 - knots[index+1].1 > 0 {
                    knots[index+1].0 += 1;
                    knots[index+1].1 += 1;
                } // north west
                else if knots[index+1].0 - knots[index].0 > 0 && knots[index].1 - knots[index+1].1 > 0 {
                    knots[index+1].0 -= 1;
                    knots[index+1].1 += 1;
                } // south east
                else if knots[index].0 - knots[index+1].0 > 0 && knots[index+1].1 - knots[index].1 > 0 {
                    knots[index+1].0 += 1;
                    knots[index+1].1 -= 1;
                } // south west 
                else {
                    knots[index+1].0 -= 1;
                    knots[index+1].1 -= 1;
                }
            }, 
            NextStep::SameDirection => {
                // Current is to the right of next:
                if knots[index].0 - knots[index+1].0 == 2 {
                    knots[index+1].0 += 1;
                } else if knots[index+1].0 - knots[index].0 == 2 {
                    // Current is to the left of next:
                    knots[index+1].0 -= 1;
                } else if knots[index].1 - knots[index+1].1 == 2 {
                    // Current is above next:
                    knots[index+1].1 += 1;
                } else {
                    knots[index+1].1 -= 1;
                }
            },
            NextStep::Stay => {
                index += 1;
                continue;
            },
            _ => panic!("Issue checking whether touching")
        }

        index += 1;
    }

    println!("{:?}", knots);
}