use itertools::Itertools;

fn solve(input: &str, start: bool) -> usize {
    let window = input.chars().collect::<Vec<char>>();

    let offset = if start { 14 } else { 4 };

    let thing = window
        .windows(offset)
        .enumerate()
        .find(|(_, c)| c.iter().unique().collect::<Vec<&char>>().len() == c.len())
        .unwrap();

    let blah = thing.0;

    blah + offset
}

fn part_one(input: &str) -> usize {
    solve(input, false)
}
fn part_two(input: &str) -> usize {
    solve(input, true)
}

pub fn main() {
    let input = include_str!("../inputs/06.txt");

    println!("part one: {}", part_one(input)); //
    println!("part two: {}", part_two(input)); //
}
