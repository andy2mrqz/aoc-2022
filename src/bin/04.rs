use std::collections::HashSet;

type Set = HashSet<u32>;

fn count_overlaps_where(input: &str, overlap_fn: &dyn Fn(&Set, &Set) -> bool) -> usize {
    input
        .lines()
        .map(|assignments| {
            let assignments: Vec<&str> = assignments.split(",").collect();

            let left_range: Vec<u32> = assignments[0].split("-").flat_map(|n| n.parse()).collect();
            let left_set: HashSet<u32> = (left_range[0]..=left_range[1]).collect();

            let right_range: Vec<u32> = assignments[1].split("-").flat_map(|n| n.parse()).collect();
            let right_set: HashSet<u32> = (right_range[0]..=right_range[1]).collect();

            (left_set, right_set)
        })
        .filter(|(set_a, set_b)| overlap_fn(set_a, set_b) || overlap_fn(set_b, set_a))
        .count()
}

fn part_one(input: &str) -> usize {
    count_overlaps_where(input, &|set_a, set_b| set_a.is_subset(set_b))
}
fn part_two(input: &str) -> usize {
    count_overlaps_where(input, &|set_a, set_b| !set_a.is_disjoint(set_b))
}

pub fn main() {
    let input = include_str!("../inputs/04.txt");

    println!("part one: {}", part_one(input)); // 524
    println!("part two: {}", part_two(input)); // 798
}
