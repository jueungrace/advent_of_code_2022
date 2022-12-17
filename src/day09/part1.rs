use std::collections::{HashMap, HashSet};

use crate::day09::{Input, Output};

use super::Direction;

pub fn solve(input: &Input) -> Output {

    let mut visited: HashMap<i64, HashSet<i64>> = HashMap::new();
    visited.insert(0, HashSet::from([0]));

    let mut head: (i64, i64) = (0, 0);

    let mut tail: (i64, i64) = (0, 0);

    let mut unique_visited_positions = 1;

    input.iter().for_each(|(direction, mut steps)| {

        // println!(">> {:?} {}", direction, steps);

        while steps > 0 {
            // Move head position: 
            match direction {
                Direction::Down => head.1 -= 1,
                Direction::Left => head.0 -= 1,
                Direction::Right => head.0 += 1,
                Direction::Up => head.1 += 1,
                _ => panic!("Issue moving head position")
            };

            // Check if head and tail are not touching:
            let tail_step = are_touching(head, tail);
            // println!("{:?}", tail_step);
            
            match tail_step {
                NextStep::Diagonal => {
                    // Head is north east:
                    if head.0 - tail.0 > 0 && head.1 - tail.1 > 0 {
                        tail.0 += 1;
                        tail.1 += 1;
                    } // north west
                    else if tail.0 - head.0 > 0 && head.1 - tail.1 > 0 {
                        tail.0 -= 1;
                        tail.1 += 1;
                    } // south east
                    else if head.0 - tail.0 > 0 && tail.1 - head.1 > 0 {
                        tail.0 += 1;
                        tail.1 -= 1;
                    } // south west 
                    else {
                        tail.0 -= 1;
                        tail.1 -= 1;
                    }
                }, 
                NextStep::SameDirection => {
                    // TODO: pull into own function move(head_or_tail, direction):
                    match direction {
                        Direction::Down => tail.1 -= 1,
                        Direction::Left => tail.0 -= 1,
                        Direction::Right => tail.0 += 1,
                        Direction::Up => tail.1 += 1
                    };
                },
                NextStep::Stay => {
                    steps -= 1;
                    continue;
                },
                _ => panic!("Issue checking whether touching")
            }

            // Check if current tail x position has been visited
            if visited.contains_key(&tail.0) {
                let mut tail_y_values = visited.get_mut(&tail.0).unwrap();
                // Tail y position not visited
                if !tail_y_values.contains(&tail.1) {
                    tail_y_values.insert(tail.1);
                    unique_visited_positions += 1;
                } 
            } else {
                // X coordinate has never been visited
                visited.insert(tail.0, HashSet::from([tail.1]));
                unique_visited_positions += 1;
            }

            // println!("{:?}", visited);

            steps -= 1;
        }
    });

    (unique_visited_positions.into())
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum NextStep {
    SameDirection,
    Diagonal,
    Stay 
}

// TODO: refactor to consider when a diff knot is already in that spot
pub fn are_touching(head: (i64, i64), tail: (i64, i64)) -> NextStep {
    // Are they at least in the same row or col?
    if head.0 == tail.0 || head.1 == tail.1 {
        // Are they apart by two steps?
        if (head.0 - tail.0).abs() == 2 || (head.1 - tail.1).abs() == 2 {
            (NextStep::SameDirection)
        } else {
            (NextStep::Stay)
        }
    } else {
        // Is the tail 1 row and 1 col away?
        if (head.0 - tail.0).abs() == 1 && (head.1 - tail.1).abs() == 1 {
            (NextStep::Stay)
        } else {
            (NextStep::Diagonal)
        }
    }
}