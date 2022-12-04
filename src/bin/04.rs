use std::collections::HashSet;

type Set = HashSet<u32>;

fn overlap_count_where(input: &str, overlap_fn: &dyn Fn(&Set, &Set) -> bool) -> usize {
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
        .filter(|(left_set, right_set)| {
            overlap_fn(left_set, right_set) || overlap_fn(right_set, left_set)
        })
        .count()
}

fn part_one(input: &str) -> usize {
    overlap_count_where(input, &|left_set, right_set| left_set.is_subset(&right_set))
}
fn part_two(input: &str) -> usize {
    overlap_count_where(input, &|left_set, right_set| {
        !left_set.is_disjoint(right_set)
    })
}

pub fn main() {
    let input = include_str!("../inputs/04.txt");

    println!("part one: {}", part_one(input)); // 524
    println!("part two: {}", part_two(input)); // 798
}
