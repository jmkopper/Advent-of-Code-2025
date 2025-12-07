use crate::util::{grid::Grid, point::Point};
use std::collections::HashMap;
use std::error::Error;

pub fn run(input: &str) -> Result<(), Box<dyn Error>> {
    let g = Grid::parse(input);

    println!("Part 1: {}", part1(&g));
    println!("Part 2: {}", part2(&g));

    Ok(())
}

fn part1(g: &Grid<u8>) -> usize {
    let mut total = 0;
    let mut beams = g.clone();
    let start = g.find(b'S').expect("No S");
    beams[start] = b'|';

    for y in 0..beams.height - 1 {
        for x in 0..beams.width {
            let p = Point { x, y };
            let v = beams[p];

            if v != b'|' {
                continue;
            }

            let below = Point { x, y: y + 1 };
            match beams[below] {
                b'.' => beams[below] = b'|',
                b'^' => {
                    // Eric made it so we don't have to check bounds
                    beams[Point { x: x - 1, y: y + 1 }] = b'|';
                    beams[Point { x: x + 1, y: y + 1 }] = b'|';
                    total += 1
                }
                _ => {}
            }
        }
    }

    total
}

fn count_paths(g: &mut Grid<u8>, pos: Point, memo: &mut HashMap<Point, usize>) -> usize {
    let mut path_count = 0;

    g[pos] = b'.';
    if pos.y == g.height - 1 {
        return 1;
    }

    if memo.contains_key(&pos) {
        return *memo.get(&pos).unwrap();
    }

    let below = Point {
        x: pos.x,
        y: pos.y + 1,
    };

    if g[below] == b'.' {
        g[below] = b'|';
        path_count += count_paths(g, below, memo);
    } else {
        let below_left = Point {
            x: pos.x - 1,
            y: pos.y,
        };
        g[below_left] = b'|';
        path_count += count_paths(g, below_left, memo);

        let below_right = Point {
            x: pos.x + 1,
            y: pos.y,
        };
        g[below_right] = b'|';
        path_count += count_paths(g, below_right, memo);
    }

    memo.insert(pos, path_count);
    path_count
}

fn part2(g: &Grid<u8>) -> usize {
    let mut beams = g.clone();
    let start_pos = g.find(b'S').unwrap();
    beams[start_pos] = b'|';
    let mut memo: HashMap<Point, usize> = HashMap::new();
    count_paths(&mut beams, start_pos, &mut memo)
}
