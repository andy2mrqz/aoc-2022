use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Monkey {
    items: VecDeque<usize>,
    op: (String, String),
    test_divisible_by: usize,
    true_recipient: usize,
    false_recipient: usize,
    inspected: usize,
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let raw_monkeys = input.split("\n\n");
    raw_monkeys
        .map(|monkey| {
            let mut attrs = monkey.lines().skip(1);
            let items = attrs.next().unwrap()[18..]
                .split(", ")
                .flat_map(|worry_level| worry_level.parse())
                .collect();
            let operation = attrs.next().unwrap()[23..].split_once(' ').unwrap();
            let operator = operation.0.to_owned();
            let operand = operation.1.to_owned();
            let test_divisible_by = attrs.next().unwrap()[21..].parse().unwrap();
            let test_true = attrs.next().unwrap()[29..].parse().unwrap();
            let test_false = attrs.next().unwrap()[30..].parse().unwrap();

            Monkey {
                items,
                op: (operator, operand),
                test_divisible_by,
                true_recipient: test_true,
                false_recipient: test_false,
                inspected: 0,
            }
        })
        .collect()
}

fn solve(input: &str) -> usize {
    let mut monkeys = parse_input(input);

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let item = monkeys[i].items.pop_front().unwrap();
                monkeys[i].inspected += 1;
                let operand: &str = monkeys[i].op.1.as_ref();
                let operand: usize = match operand {
                    "old" => item,
                    _ => operand.parse().unwrap(),
                };
                let operator: &str = monkeys[i].op.0.as_ref();
                let new_worry_level = match operator {
                    "*" => (item * operand) / 3,
                    "+" => (item + operand) / 3,
                    _ => unreachable!(),
                };
                let test_result = new_worry_level
                    .rem_euclid(monkeys[i].test_divisible_by)
                    .eq(&0);
                let recipient_id = match test_result {
                    true => monkeys[i].true_recipient,
                    false => monkeys[i].false_recipient,
                };
                let recipient = monkeys.get_mut(recipient_id).unwrap();
                recipient.items.push_back(new_worry_level);
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));
    monkeys[0].inspected * monkeys[1].inspected
}

pub fn main() {
    let input = include_str!("../inputs/11.txt");
    println!("part one: {}", solve(input)); //
                                            // println!("part two: {}", solve(input)); //
}
