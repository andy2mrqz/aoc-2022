use itertools::Itertools;
use std::collections::HashMap;

fn abs_path(path: &Vec<String>) -> String {
    format!("/{}", path.iter().skip(1).join("/"))
}

fn dir_size(dir: Vec<(usize, &str)>) -> usize {
    dir.iter().fold(0, |acc, (size, _)| acc + size)
}

fn get_sums(input: &str) -> HashMap<String, usize> {
    let mut path = Vec::new();
    let mut stats: HashMap<String, Vec<(usize, &str)>> = HashMap::new();

    for line in input.lines() {
        if line == "$ cd .." {
            path.pop();
        } else if line.starts_with("$ cd ") {
            let dir_name = line.split("$ cd ").collect::<Vec<&str>>()[1].to_owned();
            path.push(dir_name);
            stats.insert(abs_path(&path), Vec::new());
        } else if line.starts_with("dir") {
            let dir_name = line.split("dir ").collect::<Vec<&str>>()[1].to_owned();
            path.push(dir_name);
            stats.insert(abs_path(&path), Vec::new());
            path.pop();
        } else if line.starts_with(|c: char| c.is_numeric()) {
            let (size, name) = line.split(" ").collect_tuple().unwrap();
            let size: usize = size.parse().unwrap();
            let items = stats.get_mut(&abs_path(&path)).unwrap();
            items.push((size, name));
        }
    }

    let keys: Vec<&String> = stats.keys().collect();

    let mut sums: HashMap<String, usize> = HashMap::new();
    for (k, v) in stats.clone() {
        let mut sum = dir_size(v);
        for key in keys.clone() {
            if key != &k && key.starts_with(&k) {
                sum += dir_size(stats.get(key).unwrap().to_vec());
            }
        }
        sums.insert(k, sum);
    }

    sums
}

fn part_one(input: &str) -> usize {
    let sums = get_sums(input);

    let max = 100_000;
    let filtered_paths = sums
        .iter()
        .filter(|(_, size)| size <= &&max)
        .collect::<Vec<_>>();

    filtered_paths
        .iter()
        .fold(0, |acc: usize, (_, size)| acc + **size)
}

fn part_two(input: &str) -> usize {
    let sums = get_sums(input);

    let total_space = 70_000_000;
    let used_space = sums.get("/").unwrap();
    let unused_space = total_space - used_space;
    let desired_space = 30_000_000;
    let amount_to_free = desired_space - unused_space;

    let filtered_paths = sums
        .iter()
        .filter(|(_, size)| size >= &&amount_to_free)
        .collect::<Vec<_>>();

    let sizes: Vec<usize> = filtered_paths.iter().map(|(_, size)| **size).collect();

    *sizes.iter().min().unwrap()
}

pub fn main() {
    let input = include_str!("../inputs/07.txt");

    println!("part one: {}", part_one(input)); // 1315285
    println!("part two: {}", part_two(input)); //
}
