use std::collections::HashSet;

type Set = HashSet<u32>;

fn count_overlaps_where(input: &str, overlap_fn: &dyn Fn(&Set, &Set) -> bool) -> usize {
    input
        .lines()
        .filter(|assignments| {
            let assignments: Vec<u32> = assignments
                .split(['-', ','])
                .flat_map(|n| n.parse())
                .collect();

            let left_set: HashSet<u32> = (assignments[0]..=assignments[1]).collect();
            let right_set: HashSet<u32> = (assignments[2]..=assignments[3]).collect();

            overlap_fn(&left_set, &right_set) || overlap_fn(&right_set, &left_set)
        })
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
