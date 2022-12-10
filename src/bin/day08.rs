fn visible_treeline(tree: char, trees: &Vec<char>) -> Vec<char> {
    let mut taken = vec![];
    for other in trees {
        if tree.cmp(&other).is_le() {
            taken.push(*other);
            break;
        } else {
            taken.push(*other);
        }
    }
    taken
}

fn solve(input: &str) -> (usize, usize) {
    let grid: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
    let w = grid.first().unwrap().len();
    let h = grid.len();
    let perim = w * 2 + h * 2 - 4;

    let mut visible_trees = 0;
    let mut scenic_score = 0;
    for r in 1..(w - 1) {
        for c in 1..(h - 1) {
            let tree = grid[r][c];

            let left = (0..c).rev().map(|next| grid[r][next]).collect();
            let right = ((c + 1)..w).map(|next| grid[r][next]).collect();
            let up = (0..r).rev().map(|next| grid[next][c]).collect();
            let down = ((r + 1)..h).map(|next| grid[next][c]).collect();
            let neighbors: [Vec<char>; 4] = [left, right, up, down];

            // visible trees logic
            let is_visible = neighbors.iter().fold(false, |acc, treeline| {
                acc || treeline.iter().all(|other| tree.cmp(&other).is_gt())
            });
            if is_visible {
                visible_trees += 1;
            }
            // scenic score logic
            let total_score = neighbors.iter().fold(1, |acc, treeline| {
                acc * visible_treeline(tree, treeline).len()
            });

            if total_score > scenic_score {
                scenic_score = total_score
            }
        }
    }

    (visible_trees + perim, scenic_score)
}

pub fn main() {
    let input = include_str!("../inputs/08.txt");
    let answer = solve(input);

    println!("part one: {}", answer.0); // 1688
    println!("part two: {}", answer.1); // 410400
}
