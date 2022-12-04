fn parse_input(input: &str) -> Vec<bool> {
    input
        .lines()
        .map(|assignments| {
            let assignments: Vec<&str> = assignments.split(",").collect();
            let [left, right]: [&str; 2] = assignments.try_into().unwrap();

            let left_range: Vec<u32> = left.split("-").flat_map(|n| n.parse()).collect();
            let [left_start, left_end]: [u32; 2] = left_range.try_into().unwrap();
            let left_range = left_start..=left_end;

            let right_range: Vec<u32> = right.split("-").flat_map(|n| n.parse()).collect();
            let [right_start, right_end]: [u32; 2] = right_range.try_into().unwrap();
            let right_range = right_start..=right_end;


            left_range.contains(right_range.start()) &&
            left_range.contains(right_range.end()) || 
            right_range.contains(left_range.start()) &&
            right_range.contains(left_range.end())
        })
        .collect()
}

fn part_one(input: &str) -> usize {
    parse_input(input).iter().filter(|n| *n.to_owned() == true).count()
}
fn part_two(input: &str) -> u32 {
    parse_input(input);

    10
}

pub fn main() {
    let input = include_str!("../inputs/04.txt");

    println!("part one: {}", part_one(input)); // 7875
    println!("part two: {}", part_two(input)); // 2479
}
