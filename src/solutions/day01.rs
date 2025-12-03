use std::error::Error;
use std::fs;

const DIAL_SIZE: i64 = 100;
const START_POS: i64 = 50;

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("puzzles/day01.txt")?;

    // Parse "L50" to -50, "R20" to 20
    let spins: Vec<i64> = input
        .lines()
        .map(|line| {
            let (d, n) = line.trim().split_at(1);
            let amt = n.parse::<i64>()?;
            Ok(if d == "L" { -amt } else { amt })
        })
        .collect::<Result<_, Box<dyn Error>>>()?;

    let (part1, part2) = solve(&spins);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn solve(spins: &[i64]) -> (i64, i64) {
    let mut pos = START_POS;
    let mut stops_at_zero = 0;
    let mut zero_passes = 0;

    for &delta in spins {
        if delta > 0 {
            // Moving Right
            zero_passes += (pos + delta) / DIAL_SIZE;
        } else {
            // Moving Left
            let target = pos + delta;
            if target <= 0 {
                let cross = if pos == 0 { 0 } else { 1 };
                zero_passes += cross + (target.abs() / DIAL_SIZE);
            }
        }

        pos = (pos + delta).rem_euclid(DIAL_SIZE);
        // Part 1
        if pos == 0 {
            stops_at_zero += 1;
        }
    }

    (stops_at_zero, zero_passes)
}
