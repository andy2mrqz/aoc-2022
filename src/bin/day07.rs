use itertools::Itertools;
use std::collections::HashMap;

fn abs_path(path: &Vec<String>) -> String {
    format!("/{}", path.iter().skip(1).join("/"))
}

fn dir_sizes(input: &str) -> Vec<usize> {
    let mut pwd = Vec::new();
    let mut dirs: HashMap<String, Vec<usize>> = HashMap::new();

    for line in input.lines() {
        if line == "$ cd .." {
            pwd.pop();
        } else if line.starts_with("$ cd ") {
            let dir_name = line.split_once("$ cd ").unwrap().1.to_owned();
            pwd.push(dir_name);
            dirs.insert(abs_path(&pwd), Vec::new());
        } else if line.starts_with(|c: char| c.is_numeric()) {
            let size = line.split_once(" ").unwrap().0.parse().unwrap();
            let items = dirs.get_mut(&abs_path(&pwd)).unwrap();
            items.push(size);
        }
    }

    let mut sums = vec![0; dirs.keys().len()]; // preallocate dir sizes
    for (idx, dir) in dirs.keys().enumerate() {
        for subdir in dirs.keys().filter(|subdir| subdir.starts_with(dir)) {
            let subdir_contents = dirs.get(subdir).unwrap();
            sums[idx] += subdir_contents.iter().sum::<usize>()
        }
    }
    sums
}

fn part_one(input: &str, max: usize) -> usize {
    dir_sizes(input).iter().filter(|&size| size <= &max).sum()
}

fn part_two(input: &str) -> usize {
    let dirs = dir_sizes(input);
    let total_space = 70_000_000;
    let used_space = dirs.iter().max().unwrap();
    let unused_space = total_space - used_space;
    let desired_space = 30_000_000;
    let amount_to_free = desired_space - unused_space;

    *dirs
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
