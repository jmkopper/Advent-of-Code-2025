use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("puzzles/day02.txt")?;
    let ranges: Result<Vec<_>, _> = contents.trim().split(',').map(parse_line).collect();
    let ranges = ranges?;
    println!("Part 1: {}", part1(&ranges));
    println!("Part 2: {}", part2(&ranges));

    Ok(())
}

fn parse_line(line: &str) -> Result<(u64, u64), Box<dyn Error>> {
    let (left, right) = line.split_once("-").expect("bad data");
    let left_parsed = left.parse()?;
    let right_parsed = right.parse()?;
    Ok((left_parsed, right_parsed))
}

fn num_digits(x: u64) -> u32 {
    if x == 0 {
        return 1;
    }
    let mut n = x;
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
}

fn is_repeat(x: u64) -> Option<u64> {
    let n = num_digits(x);
    if n % 2 != 0 {
        return None;
    }
    let first_digits = x / (10u64.pow(n / 2));
    let last_digits = x % (10u64.pow(n / 2));

    if first_digits == last_digits {
        return Some(x);
    } else {
        return None;
    }
}

fn is_multi_repeat(x: u64) -> Option<u64> {
    let mut buf = [0u8; 20]; // handle 20 digits max
    let mut y = x; // mutable copy of x
    let mut num_digits = 0;

    // collect the digits of x backwards
    while y > 0 {
        buf[19 - num_digits] = (y % 10) as u8;
        y /= 10;
        num_digits += 1;
    }

    let digits = &buf[20 - num_digits..];
    for pattern_length in 1..=num_digits / 2 {
        if num_digits % pattern_length != 0 {
            continue;
        }

        let pattern = &digits[..pattern_length];
        if digits.chunks_exact(pattern_length).all(|c| c == pattern) {
            return Some(x);
        }
    }
    None
}

fn part1(ranges: &[(u64, u64)]) -> usize {
    let mut count = 0;
    for &(left, right) in ranges {
        count += (left..=right).filter_map(|x| is_repeat(x)).sum::<u64>();
    }

    count as usize
}

fn part2(ranges: &[(u64, u64)]) -> usize {
    let mut count = 0;
    for &(left, right) in ranges {
        count += (left..=right)
            .filter_map(|x| is_multi_repeat(x))
            .sum::<u64>();
    }

    count as usize
}
