use itertools::Itertools;

fn solve(input: &str, uniq_char_count: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();
    let marker_start_idx = chars
        .windows(uniq_char_count)
        .find_position(|c| c.iter().unique().collect::<Vec<&char>>().len() == c.len())
        .unwrap()
        .0;

    uniq_char_count + marker_start_idx
}

pub fn main() {
    let input = include_str!("../inputs/06.txt");

    println!("part one: {}", solve(input, 4)); // 1134
    println!("part two: {}", solve(input, 14)); // 2263
}
