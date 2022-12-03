use std::collections::HashSet;

fn parse_input_pt1(input: &str) -> Vec<char> {
    input
        .lines()
        .map(|line| {
            let count = line.len();

            let first_half = &line[0..(count / 2)];
            let second_half = &line[(count / 2)..];

            let first_chars: HashSet<char> = first_half.chars().collect();

            second_half
                .chars()
                .find(|c| first_chars.contains(&c))
                .unwrap()
        })
        .collect()
}

fn part_one(input: &str) -> u32 {
    parse_input_pt1(input)
        .iter()
        .map(|c| match *c as u32 {
            65..=90 => *c as u32 - 38,
            97..=122 => *c as u32 - 96,
            _ => unreachable!(),
        })
        .sum()
}

fn parse_input_pt2(input: &str) -> Vec<char> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunks| {
            let first_set: HashSet<char> = chunks[0].chars().collect();
            let second_set: HashSet<char> = chunks[1].chars().collect();
            let third_chunk = chunks[2];

            third_chunk
                .chars()
                .find(|c| first_set.contains(&c) && second_set.contains(&c))
                .unwrap()
        })
        .collect()
}

fn part_two(input: &str) -> u32 {
    parse_input_pt2(input)
        .iter()
        .map(|c| match *c as u32 {
            65..=90 => *c as u32 - 38,
            97..=122 => *c as u32 - 96,
            _ => unreachable!(),
        })
        .sum()
}

pub fn main() {
    let input = include_str!("../inputs/03.txt");

    println!("part one: {}", part_one(input)); //
    println!("part two: {}", part_two(input)); //
}
