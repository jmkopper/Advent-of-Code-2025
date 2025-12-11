use std::error::Error;

pub fn run(input: &str) -> Result<(), Box<dyn Error>> {
    let red_tiles: Vec<_> = input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(',');
            let x = parts.next()?.trim().parse::<i64>().ok()?;
            let y = parts.next()?.trim().parse::<i64>().ok()?;
            Some((x, y))
        })
        .collect();

    println!("Part 1: {}", part1(&red_tiles));
    println!("Part 2: {}", part2(&red_tiles));

    Ok(())
}

fn area(a: (i64, i64), b: (i64, i64)) -> i64 {
    (1 + (a.0 - b.0).abs()) * (1 + (a.1 - b.1).abs())
}

fn part1(reds: &[(i64, i64)]) -> i64 {
    let mut biggest = 0;
    for i in 0..reds.len() {
        for j in i + 1..reds.len() {
            biggest = biggest.max(area(reds[i], reds[j]))
        }
    }
    biggest
}

struct Edge {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
}

impl Edge {
    fn intersects(&self, a: (i64, i64), b: (i64, i64)) -> bool {
        let (min_x, max_x) = sort(a.0, b.0);
        let (min_y, max_y) = sort(a.1, b.1);
        // we only care about transverse intersections
        min_x < self.x2 && max_x > self.x1 && min_y < self.y2 && max_y > self.y1
    }
}

fn sort(a: i64, b: i64) -> (i64, i64) {
    if a < b { (a, b) } else { (b, a) }
}

fn build_edges(reds: &[(i64, i64)]) -> Vec<Edge> {
    let mut edges = Vec::new();

    for i in 0..reds.len() - 1 {
        let (x1, x2) = sort(reds[i].0, reds[i + 1].0);
        let (y1, y2) = sort(reds[i].1, reds[i + 1].1);
        edges.push(Edge { x1, y1, x2, y2 });
    }

    // close the loop
    let (x1, x2) = sort(reds[0].0, reds[reds.len() - 1].0);
    let (y1, y2) = sort(reds[0].1, reds[reds.len() - 1].1);
    edges.push(Edge { x1, y1, x2, y2 });

    edges
}

pub fn part2(red_tiles: &[(i64, i64)]) -> i64 {
    let edges = build_edges(red_tiles);
    let mut biggest = 0;

    for i in 0..red_tiles.len() - 1 {
        for j in i..red_tiles.len() {
            let a = red_tiles[i];
            let b = red_tiles[j];

            if edges.iter().any(|e| e.intersects(a, b)) {
                continue;
            }

            let area = area((a.0, a.1), (b.0, b.1));
            biggest = biggest.max(area);
        }
    }

    biggest
}
