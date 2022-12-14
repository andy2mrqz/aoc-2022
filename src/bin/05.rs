use itertools::Itertools;
use std::{collections::HashMap, iter::Rev, str::Lines};

fn parse_stacks(stacks: Rev<Lines>) -> HashMap<usize, Vec<char>> {
    let mut map = HashMap::new();
    for (idx, line) in stacks.enumerate() {
        if idx == 0 {
            for stack_num in line.split_whitespace() {
                map.insert(stack_num.parse().unwrap(), vec![]);
            }
        }
        for (i, item) in line.chars().skip(1).step_by(4).enumerate() {
            if item.is_alphabetic() {
                map.get_mut(&(i + 1)).map(|stack| stack.push(item));
            }
        }
    }
    map
}

fn solve(input: &str, one_at_a_time: bool) -> String {
    let (stacks, instructions) = input.split("\n\n").collect_tuple().unwrap();
    let mut stacks = parse_stacks(stacks.lines().rev());

    for instruction in instructions.lines() {
        let (_, qty, _, from, _, to) = instruction.split_whitespace().collect_tuple().unwrap();
        let qty = qty.parse().unwrap();

        let mut items_to_move: Vec<char> = vec![];
        {
            let from = stacks.get_mut(&from.parse().unwrap()).unwrap();
            for _ in 0..qty {
                items_to_move.push(from.pop().unwrap());
            }
        }
        {
            if one_at_a_time {
                items_to_move.reverse()
            }
            let to = stacks.get_mut(&to.parse().unwrap()).unwrap();
            for _ in 0..qty {
                to.push(items_to_move.pop().unwrap());
            }
        }
    }

    let mut res: String = "".to_string();
    for stack_idx in 1..=stacks.len() {
        let top = stacks.get_mut(&stack_idx).unwrap().pop().unwrap();
        res = format!("{}{}", res, top);
    }
    res
}

pub fn main() {
    let input = include_str!("../inputs/05.txt");

    println!("part one: {}", solve(input, true)); // TLNGFGMFN
    println!("part two: {}", solve(input, false)); // FGLQJCMBD
}
