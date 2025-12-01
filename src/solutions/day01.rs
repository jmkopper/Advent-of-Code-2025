use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

const DIAL_SIZE: i64 = 100;
const START_POS: i64 = 50;
type Spin = (i64, i64);

pub fn run() -> Result<(), Box<dyn Error>> {
    let file = File::open("puzzles/day01.txt")?;
    let reader = io::BufReader::new(file);
    let spins: Result<Vec<_>, _> = reader.lines().map(|line| parse_line(&line?)).collect();
    let spins = spins?;
    println!("Part 1: {}", part1(&spins));
    println!("Part 2: {}", part2(&spins));

    Ok(())
}

fn parse_line(line: &str) -> Result<Spin, Box<dyn Error>> {
    let (dir_char, num_str) = line.split_at(1);
    let dir = match dir_char {
        "L" => -1,
        "R" => 1,
        _ => 1,
    };

    let amount = num_str.parse::<i64>()?;
    Ok((dir, amount))
}

fn turn_dial(dial: i64, spin: Spin) -> i64 {
    (dial + spin.0 * spin.1).rem_euclid(DIAL_SIZE)
}

fn part1(spins: &[Spin]) -> i64 {
    spins
        .iter()
        .scan(START_POS, |p, &s| {
            *p = turn_dial(*p, s);
            Some(*p)
        })
        .filter(|&p| p == 0)
        .count() as i64
}

fn count_left_zeros(pos: i64, amt: i64) -> i64 {
    let end_pos = pos - amt;
    if end_pos > 0 {
        return 0;
    }
    let t = if pos == 0 { 0 } else { 1 };

    return t + (end_pos.abs() / DIAL_SIZE);
}

fn part2(spins: &[(i64, i64)]) -> i64 {
    let mut pos = START_POS;
    let mut count = 0;

    for &(dir, amt) in spins {
        if dir > 0 {
            count += (pos + amt) / DIAL_SIZE;
            pos = (pos + amt) % DIAL_SIZE;
        } else {
            count += count_left_zeros(pos, amt);
            pos = (pos - amt).rem_euclid(DIAL_SIZE);
        }
    }
    count
}
