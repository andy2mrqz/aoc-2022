#[derive(Copy, Clone, Eq, PartialEq)]
struct Dist {
    steps: usize,
    position: (usize, usize),
}

fn neighbors(cell: (usize, usize), w: usize, h: usize) -> Vec<(usize, usize)> {
    let mut n = Vec::new();
    if cell.0 as i8 - 1 >= 0 {
        n.push((cell.0 - 1, cell.1));
    }
    if cell.0 + 1 <= (h - 1) {
        n.push((cell.0 + 1, cell.1));
    }
    if cell.1 as i8 - 1 >= 0 {
        n.push((cell.0, cell.1 - 1));
    }
    if cell.1 + 1 <= (w - 1) {
        n.push((cell.0, cell.1 + 1));
    }
    n
}

fn height(grid: &Vec<Vec<char>>, position: (usize, usize)) -> char {
    let curr_height = grid[position.0][position.1];
    match curr_height {
        'S' => 'a',
        'E' => 'z',
        _ => curr_height,
    }
}

fn solve(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> usize {
    let w = grid.first().unwrap().len();
    let h = grid.len();

    let mut dist = vec![vec![usize::MAX; w]; h];
    dist[start.0][start.1] = 0;

    let mut queue = Vec::new();
    queue.push(Dist {
        steps: 0,
        position: start,
    });

    while let Some(Dist { steps, position }) = queue.pop() {
        if steps > dist[position.0][position.1] {
            continue;
        }
        let curr_height = height(&grid, position);
        for neighbor in neighbors(position, w, h) {
            let height_diff = height(&grid, neighbor) as i8 - curr_height as i8;
            if height_diff > 1 {
                continue;
            }
            let next = Dist {
                steps: steps + 1,
                position: neighbor,
            };
            if next.steps < dist[neighbor.0][neighbor.1] {
                queue.push(next);
                dist[neighbor.0][neighbor.1] = next.steps;
            }
        }
    }

    dist[end.0][end.1]
}

pub fn main() {
    let input = include_str!("../inputs/12.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut starting_options = Vec::new();

    for r in 0..grid.len() {
        for c in 0..grid.first().unwrap().len() {
            match grid[r][c] {
                'S' => {
                    start = (r, c);
                    starting_options.push((r, c));
                }
                'a' => starting_options.push((r, c)),
                'E' => end = (r, c),
                _ => {}
            }
        }
    }

    println!("part one: {}", solve(&grid, start, end)); // 408

    // probably a faster algorithm but this worked with --release 🙈
    let mut fewest_steps = usize::MAX;
    for option in starting_options {
        let steps = solve(&grid, option, end);
        fewest_steps = std::cmp::min(steps, fewest_steps);
    }
    println!("part two: {}", fewest_steps); //
}
