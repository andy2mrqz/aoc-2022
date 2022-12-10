use std::collections::HashSet;

fn parse_input_pt1(input: &str) -> Vec<char> {
    input
        .lines()
        .flat_map(|line| {
            let half = line.len() / 2;
            let first_set: HashSet<char> = line[..half].chars().collect();
            let second_half = &line[half..];

            second_half.chars().find(|c| first_set.contains(&c))
        })
        .collect()
}

fn parse_input_pt2(input: &str) -> Vec<char> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .flat_map(|chunks| {
            let first_set: HashSet<char> = chunks[0].chars().collect();
            let second_set: HashSet<char> = chunks[1].chars().collect();
            let third_chunk = chunks[2];

            third_chunk
                .chars()
                .find(|c| first_set.contains(&c) && second_set.contains(&c))
        })
        .collect()
}

fn sum_priorities(items: Vec<char>) -> u32 {
    items
        .iter()
        .map(|c| match *c {
            'A'..='Z' => *c as u32 - 'A' as u32 + 27, // 'A' has priority 27
            'a'..='z' => *c as u32 - 'a' as u32 + 1,  // 'a' has priority 1
            _ => unreachable!(),
        })
        .sum()
}

fn part_one(input: &str) -> u32 {
    sum_priorities(parse_input_pt1(input))
}
fn part_two(input: &str) -> u32 {
    sum_priorities(parse_input_pt2(input))
}

pub fn main() {
    let input = include_str!("../inputs/03.txt");

    println!("part one: {}", part_one(input)); // 7875
    println!("part two: {}", part_two(input)); // 2479
}
