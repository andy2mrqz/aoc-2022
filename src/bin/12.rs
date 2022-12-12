#[derive(Copy, Clone, Eq, PartialEq)]
struct Dist {
    steps: usize,
    position: (usize, usize),
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
    let grid: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
    let w = grid.first().unwrap().len();
    let h = grid.len();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for r in 0..h {
        for c in 0..w {
            if grid[r][c] == 'S' {
                start = (r, c);
            }
            if grid[r][c] == 'E' {
                end = (r, c);
            }
        }
    }

    (grid, start, end)
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

fn solve(grid: Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> usize {
    let mut queue = Vec::new();

    let w = grid.first().unwrap().len();
    let h = grid.len();
    let mut dist = vec![vec![usize::MAX; w]; h];

    dist[start.0][start.1] = 0;
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
    let parsed = parse_input(input);
    println!("part one: {}", solve(parsed.0, parsed.1, parsed.2)); //
                                                                   // println!("part two: {}", solve(input)); //
}
