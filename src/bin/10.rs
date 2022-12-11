fn solve(input: &str) -> i32 {
    let instructions: Vec<(i32, i32)> = input
        .lines()
        .map(|line| {
            if line == "noop" {
                (1, 0)
            } else {
                (2, line[5..].parse().unwrap())
            }
        })
        .collect();

    let mut signal_strength = Vec::new();
    let mut x = 1;
    let mut cycle: i32 = 1;

    for (cost, value) in instructions {
        for i in 1..=cost {
            if cycle.rem_euclid(40) == 20 {
                signal_strength.push(cycle * x);
            }
            cycle += 1;
            if i == cost {
                x += value;
            }
        }
    }

    signal_strength.iter().sum()
}

pub fn main() {
    let input = include_str!("../inputs/10.txt");
    let answer = solve(input);

    println!("part one: {}", answer); // 13820
    println!("part two: {}", answer); //
}
