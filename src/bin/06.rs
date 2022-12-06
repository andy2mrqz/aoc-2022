use itertools::Itertools;

fn solve(input: &str) -> usize {
    let window = input.chars().collect::<Vec<char>>();

    let thing = window
        .windows(4)
        .enumerate()
        .find(|(_, c)| c.iter().unique().collect::<Vec<&char>>().len() == c.len())
        .unwrap();

    let blah = thing.0;

    blah + 4
}

fn part_one(input: &str) -> usize {
    solve(input)
}
fn part_two(input: &str) -> usize {
    solve(input);

    10
}

pub fn main() {
    let input = include_str!("../inputs/06.txt");

    println!("part one: {}", part_one(input)); //
    println!("part two: {}", part_two(input)); //
}
