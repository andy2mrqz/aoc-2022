use std::str::FromStr;

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
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
        if self == &opponent {
            Score::Draw
        } else if self.wins_against() == opponent {
            Score::Win
        } else {
            Score::Lose
        }
    }

    fn wins_against(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }

    fn loses_against(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }

    fn hand_to_play(desired_score: Score, opponent: Hand) -> Hand {
        match desired_score {
            Score::Draw => opponent,
            Score::Win => opponent.loses_against(),
            Score::Lose => opponent.wins_against(),
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

fn play(input: &str, score_fn: &dyn Fn(Vec<&str>) -> u32) -> u32 {
    input
        .lines()
        .map(|line| score_fn(line.split(' ').collect()))
        .sum()
}

fn part_one(input: &str) -> u32 {
    play(input, &|chars: Vec<&str>| {
        let opponent: Hand = chars[0].parse().unwrap();
        let you: Hand = chars[1].parse().unwrap();

        you as u32 + you.score_vs(opponent) as u32
    })
}

fn part_two(input: &str) -> u32 {
    play(input, &|chars: Vec<&str>| {
        let opponent: Hand = chars[0].parse().unwrap();
        let planned_score: Score = chars[1].parse().unwrap();

        planned_score as u32 + Hand::hand_to_play(planned_score, opponent) as u32
    })
}

pub fn main() {
    let input = include_str!("../inputs/02.txt");

    println!("part one: {}", part_one(input)); // 11666
    println!("part two: {}", part_two(input)); // 12767
}
