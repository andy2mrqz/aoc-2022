use std::collections::HashMap;

fn parse_stacks(stacks: &str) -> HashMap<usize, Vec<char>> {
    let mut map = HashMap::new();

    for (line_idx, line) in stacks.lines().rev().enumerate() {
        if line_idx == 0 {
            for (stack_idx, _) in line.split_whitespace().enumerate() {
                map.insert(stack_idx + 1, vec![]);
            }
            continue;
        }

        for i in 1..=(map.keys().len()) {
            let item = line.chars().nth(((i - 1) * 4) + 1).unwrap();
            if item != ' ' {
                let stack = map.get_mut(&i).unwrap();
                stack.push(item);
            }
        }
    }

    map
}

fn solve(input: &str) -> String {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut stacks = parse_stacks(parts[0]);

    for instruction in parts[1].lines() {
        let parts: Vec<&str> = instruction.split_whitespace().collect();
        let qty: usize = parts[1].parse().unwrap();

        let mut to_move: Vec<char> = vec![];
        {
            let from = stacks.get_mut(&parts[3].parse().unwrap()).unwrap();
            for _ in 0..qty {
                to_move.push(from.pop().unwrap());
            }
        }
        {
            let to = stacks.get_mut(&parts[5].parse().unwrap()).unwrap();
            for _ in 0..qty {
                to.push(to_move.pop().unwrap());
            }
        }
    }

    let mut res: String = "".to_string();
    for stack_idx in 1..=stacks.len() {
        res = format!(
            "{}{}",
            res,
            stacks.get_mut(&stack_idx).unwrap().pop().unwrap()
        );
    }

    res
}

fn part_one(input: &str) -> String {
    solve(input)
}
fn part_two(input: &str) -> &str {
    solve(input);

    "MAC"
}

pub fn main() {
    let input = include_str!("../inputs/05.txt");

    println!("part one: {}", part_one(input)); //
    println!("part two: {}", part_two(input)); //
}
