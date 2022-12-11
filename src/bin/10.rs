fn solve(instructions: Vec<(usize, i32)>) -> (i32, Vec<char>) {
    let mut signal_strength = 0;
    let mut x = 1;
    let mut cycle: i32 = 1;
    let mut crt = vec!['.'; 240];

    for (cost, value) in instructions {
        for _ in 1..=cost {
            if cycle.rem_euclid(40) == 20 {
                signal_strength += cycle * x;
            }
            if (x..=(x + 2)).contains(&(cycle).rem_euclid(40)) {
                crt[cycle as usize - 1] = '#';
            }
            cycle += 1;
        }
        x += value;
    }

    (signal_strength, crt)
}

pub fn main() {
    let input = include_str!("../inputs/10.txt");
    let instructions = input
        .lines()
        .map(|line| match line {
            "noop" => (1, 0),
            _ => (2, line[5..].parse().unwrap()),
        })
        .collect();

    let answers = solve(instructions);
    println!("part one: {}", answers.0); // 13820
    println!("part two"); // ZKGRKGRK
    for line in answers.1.chunks(40) {
        println!("{:?}", String::from_iter(line))
    }
}
