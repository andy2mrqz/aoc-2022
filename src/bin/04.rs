use std::ops::RangeInclusive;

type Range = RangeInclusive<u32>;

fn overlap_count_where(input: &str, overlap_fn: &dyn Fn(&(Range, Range)) -> bool) -> usize {
    input
        .lines()
        .map(|assignments| {
            let assignments: Vec<&str> = assignments.split(",").collect();

            let left_range: Vec<u32> = assignments[0].split("-").flat_map(|n| n.parse()).collect();
            let left_range = left_range[0]..=left_range[1];

            let right_range: Vec<u32> = assignments[1].split("-").flat_map(|n| n.parse()).collect();
            let right_range = right_range[0]..=right_range[1];

            (left_range, right_range)
        })
        .filter(overlap_fn)
        .count()
}

fn part_one(input: &str) -> usize {
    overlap_count_where(input, &|(left_range, right_range)| {
        left_range.contains(right_range.start()) && left_range.contains(right_range.end())
            || right_range.contains(left_range.start()) && right_range.contains(left_range.end())
    })
}
fn part_two(input: &str) -> usize {
    overlap_count_where(input, &|(left_range, right_range)| {
        left_range.contains(right_range.start())
            || left_range.contains(right_range.end())
            || right_range.contains(left_range.start())
            || right_range.contains(left_range.end())
    })
}

pub fn main() {
    let input = include_str!("../inputs/04.txt");

    println!("part one: {}", part_one(input)); // 524
    println!("part two: {}", part_two(input)); // 798
}
