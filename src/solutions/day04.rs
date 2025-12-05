use std::error::Error;

pub fn run(input: &str) -> Result<(), Box<dyn Error>> {
    let lines: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|s| s.as_bytes().to_vec())
        .collect();
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(lines));

    Ok(())
}

fn count_neighbors(grid: &[Vec<u8>], row: usize, col: usize) -> usize {
    let height = grid.len();
    let width = grid[row].len();
    let mut count = 0;

    for dr in [-1, 0, 1] {
        for dc in [-1, 0, 1] {
            if dr == 0 && dc == 0 {
                continue;
            }

            let nr = row as isize + dr;
            let nc = col as isize + dc;

            // Bounds check
            if nr < 0 || nr as usize >= height {
                continue;
            }
            if nc < 0 || nc as usize >= width {
                continue;
            }

            if grid[nr as usize][nc as usize] == b'@' {
                count += 1;
            }
        }
    }

    count
}

fn part1(lines: &[Vec<u8>]) -> usize {
    let mut count = 0;

    for row in 0..lines.len() {
        for col in 0..lines[row].len() {
            if lines[row][col] == b'@' && count_neighbors(lines, row, col) < 4 {
                count += 1;
            }
        }
    }

    count
}

fn part2(mut grid: Vec<Vec<u8>>) -> usize {
    let mut total_removed = 0;

    loop {
        let mut removed_this_round = 0;
        let height = grid.len();

        for row in 0..height {
            let width = grid[row].len();
            for col in 0..width {
                if grid[row][col] == b'@' && count_neighbors(&grid, row, col) < 4 {
                    grid[row][col] = b'.';
                    removed_this_round += 1;
                }
            }
        }

        if removed_this_round == 0 {
            break;
        }

        total_removed += removed_this_round;
    }

    total_removed
}
