use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

fn two_steps_apart(head: Coord, tail: Coord) -> bool {
    (head.y - tail.y >= 2)
        || (tail.y - head.y >= 2)
        || (head.x - tail.x >= 2)
        || (tail.x - head.x >= 2)
}

fn diagonal(head: Coord, tail: Coord) -> bool {
    (head != tail) && two_steps_apart(head, tail)
}

fn solve(input: &str) -> usize {
    let instructions: Vec<(char, u32)> = input
        .lines()
        .map(|i| {
            let (dir, steps) = i.split_once(" ").unwrap();
            (dir.chars().nth(0).unwrap(), steps.parse().unwrap())
        })
        .collect();
    //              x, y
    let mut head = Coord { x: 0, y: 0 };
    let mut tail = Coord { x: 0, y: 0 };

    let mut visited: HashSet<_> = HashSet::new();
    visited.insert(tail);

    for (dir, steps) in instructions {
        for _ in 0..steps {
            match dir {
                'R' => {
                    head.x += 1;
                    if two_steps_apart(head, tail) && head.y == tail.y {
                        tail.x += 1;
                    } else if diagonal(head, tail) {
                        tail.x += 1;
                        tail.y += if head.y > tail.y { 1 } else { -1 }
                    }
                }
                'L' => {
                    head.x -= 1;
                    if two_steps_apart(head, tail) && head.y == tail.y {
                        tail.x -= 1;
                    } else if diagonal(head, tail) {
                        tail.x -= 1;
                        tail.y += if head.y > tail.y { 1 } else { -1 }
                    }
                }
                'U' => {
                    head.y += 1;
                    if two_steps_apart(head, tail) && head.x == tail.x {
                        tail.y += 1;
                    } else if diagonal(head, tail) {
                        tail.y += 1;
                        tail.x += if head.x > tail.x { 1 } else { -1 }
                    }
                }
                'D' => {
                    head.y -= 1;
                    if two_steps_apart(head, tail) && head.x == tail.x {
                        tail.y -= 1;
                    } else if diagonal(head, tail) {
                        tail.y -= 1;
                        tail.x += if head.x > tail.x { 1 } else { -1 }
                    }
                }
                _ => {}
            }
            visited.insert(tail);
        }
    }

    visited.len()
}

pub fn main() {
    let input = include_str!("../inputs/09.txt");

    println!("part one: {}", solve(input)); //
    println!();
    // println!("part two: {}", solve(input)); //
}
