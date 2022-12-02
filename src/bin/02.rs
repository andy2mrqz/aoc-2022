//
// A rock
// B paper
// C scissors
// X lose
// Y draw
// Z win
//
// score:
// 1 rock
// 2 paper
// 3 scissors
// 0 lost
// 3 draw
// 6 win

fn judge(opponent: &str, you: &str) -> u32 {
    match opponent {
        "A" => match you {
            "X" => 3,
            "Y" => 6,
            "Z" => 0,
            _ => unreachable!(),
        },
        "B" => match you {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => unreachable!(),
        },
        "C" => match you {
            "X" => 6,
            "Y" => 0,
            "Z" => 3,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn score(x: &str) -> u32 {
    match x {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => unreachable!(),
    }
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let chars: Vec<&str> = line.split(' ').collect();

            let opponent = chars[0];
            let you = chars[1];

            judge(opponent, you) + score(you)
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let chars: Vec<&str> = line.split(' ').collect();

            let opponent = chars[0];
            let ending = chars[1];

            match ending {
                // lose
                "X" => match opponent {
                    "A" => 3,
                    "B" => 1,
                    "C" => 2,
                    _ => unreachable!(),
                },
                // draw
                "Y" => {
                    3 + match opponent {
                        "A" => 1,
                        "B" => 2,
                        "C" => 3,
                        _ => unreachable!(),
                    }
                }
                "Z" => {
                    6 + match opponent {
                        "A" => 2,
                        "B" => 3,
                        "C" => 1,
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        })
        .sum()
}

pub fn main() {
    let input = include_str!("../inputs/02.txt");

    println!("part one: {}", part_one(input)); // 11666
    println!("part two: {}", part_two(input)); // 12767
}
