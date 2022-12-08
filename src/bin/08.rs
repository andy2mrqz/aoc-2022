use std::cmp::Ordering;

fn solve(input: &str) -> usize {
    let w = input.split_once("\n").unwrap().0.len();
    let h = input.lines().count();

    let grid: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

    let mut sum = 0;
    for r in 0..w {
        for c in 0..h {
            let tree = grid[r][c];

            let left: Vec<char> = (0..c).map(|next| grid[r][next]).collect();
            let right: Vec<char> = ((c + 1)..w).map(|next| grid[r][next]).collect();
            let up: Vec<char> = (0..r).map(|next| grid[next][c]).collect();
            let down: Vec<char> = ((r + 1)..h).map(|next| grid[next][c]).collect();

            if left
                .iter()
                .all(|other| tree.cmp(other) == Ordering::Greater)
                || right
                    .iter()
                    .all(|other| tree.cmp(other) == Ordering::Greater)
                || up.iter().all(|other| tree.cmp(other) == Ordering::Greater)
                || down
                    .iter()
                    .all(|other| tree.cmp(other) == Ordering::Greater)
            {
                sum += 1
            }
        }
    }

    sum
}

pub fn main() {
    let input = include_str!("../inputs/08.txt");

    println!("part one: {}", solve(input)); // 1688
    // println!("part two: {}", solve(input)); //
}
