use std::str::FromStr;

#[repr(u32)]
#[derive(Clone, Copy)]
// The value is the score for playing the given hand
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // second part for part 1
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err("Invalid".to_owned()),
        }
    }
}

impl Hand {
    fn score_vs(&self, opponent: Hand) -> Score {
        match self {
            Hand::Rock => match opponent {
                Hand::Rock => Score::Draw,
                Hand::Paper => Score::Lose,
                Hand::Scissors => Score::Win,
            },
            Hand::Paper => match opponent {
                Hand::Rock => Score::Win,
                Hand::Paper => Score::Draw,
                Hand::Scissors => Score::Lose,
            },
            Hand::Scissors => match opponent {
                Hand::Rock => Score::Lose,
                Hand::Paper => Score::Win,
                Hand::Scissors => Score::Draw,
            },
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
enum Score {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for Score {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err("Invalid".to_owned()),
        }
    }
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let chars: Vec<&str> = line.split(' ').collect();

            let opponent: Hand = chars[0].parse().unwrap();
            let you: Hand = chars[1].parse().unwrap();

            you as u32 + you.score_vs(opponent) as u32
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let chars: Vec<&str> = line.split(' ').collect();

            let opponent: Hand = chars[0].parse().unwrap();
            let ending: Score = chars[1].parse().unwrap();

            ending as u32
                + match ending {
                    Score::Draw => opponent,
                    Score::Lose => match opponent {
                        Hand::Rock => Hand::Scissors,
                        Hand::Paper => Hand::Rock,
                        Hand::Scissors => Hand::Paper,
                    },
                    Score::Win => match opponent {
                        Hand::Rock => Hand::Paper,
                        Hand::Paper => Hand::Scissors,
                        Hand::Scissors => Hand::Rock,
                    },
                } as u32
        })
        .sum()
}

pub fn main() {
    let input = include_str!("../inputs/02.txt");

    println!("part one: {}", part_one(input)); // 11666
    println!("part two: {}", part_two(input)); // 12767
}
