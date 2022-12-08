use itertools::Itertools;
use std::collections::HashMap;

fn abs_path(path: &Vec<String>) -> String {
    format!("/{}", path.iter().skip(1).join("/"))
}

fn get_sums(input: &str) -> Vec<usize> {
    let mut path = Vec::new();
    let mut stats: HashMap<String, Vec<usize>> = HashMap::new();

    for line in input.lines() {
        if line == "$ cd .." {
            path.pop();
        } else if line.starts_with("$ cd ") {
            let dir_name = line.split_once("$ cd ").unwrap().1.to_owned();
            path.push(dir_name);
            stats.insert(abs_path(&path), Vec::new());
        } else if line.starts_with(|c: char| c.is_numeric()) {
            let size = line.split_once(" ").unwrap().0.parse().unwrap();
            let items = stats.get_mut(&abs_path(&path)).unwrap();
            items.push(size);
        }
    }

    let mut sums = vec![0; stats.keys().len()]; // preallocate dir sizes
    for (idx, dir) in stats.keys().enumerate() {
        for subdir in stats.keys().filter(|subdir| subdir.starts_with(dir)) {
            sums[idx] += stats.get(subdir).unwrap().to_vec().iter().sum::<usize>()
        }
    }
    sums
}

fn part_one(input: &str, max: usize) -> usize {
    get_sums(input).iter().filter(|&size| size <= &max).sum()
}

fn part_two(input: &str) -> usize {
    let sums = get_sums(input);
    let total_space = 70_000_000;
    let used_space = sums.iter().max().unwrap();
    let unused_space = total_space - used_space;
    let desired_space = 30_000_000;
    let amount_to_free = desired_space - unused_space;

    *sums
        .iter()
        .filter(|&size| size >= &amount_to_free)
        .min()
        .unwrap()
}

pub fn main() {
    let input = include_str!("../inputs/07.txt");

    println!("part one: {}", part_one(input, 100_000)); // 1315285
    println!("part two: {}", part_two(input)); // 9847279
}
