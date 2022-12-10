fn elves_calories(input: &str) -> Vec<Vec<u32>> {
    let elf_lists: Vec<&str> = input.split("\n\n").collect();

    elf_lists
        .iter()
        .map(|elf_list| {
            elf_list
                .lines()
                .map(|entry| entry.parse().unwrap())
                .collect()
        })
        .collect()
}

// Find elf carrying the most calories
fn part_one(input: &str) -> u32 {
    elves_calories(input)
        .iter()
        .map(|calories| calories.iter().sum())
        .max()
        .unwrap()
}

// Sum calories of top 3 elves
fn part_two(input: &str) -> u32 {
    let mut totals: Vec<u32> = elves_calories(input)
        .iter()
        .map(|calories| calories.iter().sum())
        .collect();
    totals.sort_by(|a, b| b.cmp(a));

    totals.iter().take(3).sum()
}

pub fn main() {
    let input = include_str!("../inputs/01.txt");

    println!("part one: {}", part_one(input)); // 66487
    println!("part two: {}", part_two(input)); // 197301
}
