//
// A rock
// B paper
// C scissors
// X rock
// Y paper
// Z scissors
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

fn play(opponent: &str, you: &str) -> u32 {
    judge(opponent, you) + score(you)
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| {
            let chars: Vec<&str> = line.split(' ').collect();

            let opponent = chars[0];
            let you = chars[1];

            play(opponent, you)
        })
        .collect()
}

fn part_one(input: &str) -> u32 {
    parse_input(input).iter().sum()
}

fn part_two(input: &str) -> u32 {
    parse_input(input);

    10
}

pub fn main() {
    let input = include_str!("../inputs/02.txt");

    println!("part one: {}", part_one(input));
    println!("part two: {}", part_two(input));
}
