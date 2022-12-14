use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

fn two_steps_apart(head: &Coord, tail: &Coord) -> bool {
    ((head.y - tail.y).abs() >= 2) || ((head.x - tail.x).abs() >= 2)
}

fn solve(instructions: &Vec<(char, u32)>, mut knots: Vec<Coord>) -> (usize, usize) {
    let mut visited_part_one: HashSet<_> = HashSet::new();
    let mut visited_part_two: HashSet<_> = HashSet::new();
    visited_part_one.insert(knots[0]);
    visited_part_two.insert(knots[0]);

    for (dir, steps) in instructions {
        for _ in 0..*steps {
            let head = knots.get_mut(0).unwrap();
            match dir {
                'R' => head.x += 1,
                'L' => head.x -= 1,
                'U' => head.y += 1,
                'D' => head.y -= 1,
                _ => {}
            }
            for idx in 1..knots.len() {
                let next = knots[idx - 1];
                let curr = knots.get_mut(idx).unwrap();
                if two_steps_apart(&next, curr) {
                    if next.x == curr.x {
                        curr.y += if next.y > curr.y { 1 } else { -1 }
                    } else if next.y == curr.y {
                        curr.x += if next.x > curr.x { 1 } else { -1 }
                    } else if &next != curr {
                        curr.x += if next.x > curr.x { 1 } else { -1 };
                        curr.y += if next.y > curr.y { 1 } else { -1 };
                    }
                }
            }
            visited_part_one.insert(*knots.get(1).unwrap());
            visited_part_two.insert(*knots.get(9).unwrap());
        }
    }
    (visited_part_one.len(), visited_part_two.len())
}

pub fn main() {
    let input = include_str!("../inputs/09ex.txt");
    let instructions: Vec<(_, u32)> = input
        .lines()
        .map(|i| (i.chars().nth(0).unwrap(), i[2..].parse().unwrap()))
        .collect();

    let starting_knot = Coord { x: 0, y: 0 };
    let answer = solve(&instructions, vec![starting_knot; 10]);

    println!("part one: {}", answer.0); // 5619
    println!("part two: {}", answer.1); // 2376
}
