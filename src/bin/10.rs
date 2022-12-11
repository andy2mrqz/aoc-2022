fn part_one(instructions: Vec<(usize, i32)>) -> i32 {
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

fn part_two(instructions: Vec<(usize, i32)>) -> () {
    let mut x: i32 = 1;
    let mut cycle: i32 = 1;
    let mut crt = vec!['.'; 240];

    for (cost, value) in instructions {
        for i in 1..=cost {
            if ((x - 1)..=(x + 1)).contains(&(cycle - 1).rem_euclid(40)) {
                crt[cycle as usize - 1] = '#';
            }
            cycle += 1;
            if i == cost {
                x += value;
            }
        }
    }

    for line in crt.chunks(40) {
        println!("{:?}", String::from_iter(line))
    }
}

pub fn main() {
    let input = include_str!("../inputs/10.txt");
    let instructions: Vec<(usize, i32)> = input
        .lines()
        .map(|line| {
            if line == "noop" {
                (1, 0)
            } else {
                (2, line[5..].parse().unwrap())
            }
        })
        .collect();

    println!("part one: {}", part_one(instructions.clone())); // 13820
    println!("part two");
    part_two(instructions); // ZKGRKGRK
}
