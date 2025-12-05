use std::error::Error;

pub fn run(input: &str) -> Result<(), Box<dyn Error>> {
    let (first, second) = input.split_once("\n\n").ok_or("where chunks")?;

    let mut fresh_ranges = parse_ranges(first)?;
    fresh_ranges.sort_unstable(); // tuples are sorted by dictionary order
    let ingredient_ids: Vec<usize> = second.lines().map(str::parse).collect::<Result<_, _>>()?;

    println!("Part 1: {}", part1(&fresh_ranges, &ingredient_ids));
    println!("Part 2: {}", part2(&fresh_ranges));

    Ok(())
}

fn parse_ranges(s: &str) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
    let mut ranges = Vec::new();
    for line in s.lines() {
        let (left, right) = line.split_once("-").ok_or("no hyphen")?;
        let left_int: usize = str::parse(left)?;
        let right_int: usize = str::parse(right)?;
        ranges.push((left_int, right_int));
    }
    Ok(ranges)
}

fn part1(fresh_ranges: &[(usize, usize)], ingredients: &[usize]) -> usize {
    ingredients
        .iter()
        .filter(|&&x| {
            fresh_ranges
                .iter()
                .any(|&(left, right)| x >= left && x <= right)
        })
        .count()
}

fn collapse(ranges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut new_ranges = Vec::new();
    let mut cur = ranges[0];

    for &(left, right) in &ranges[1..] {
        if left <= cur.1 {
            cur.1 = cur.1.max(right);
        } else {
            new_ranges.push(cur);
            cur = (left, right);
        }
    }

    new_ranges.push(cur);
    new_ranges
}

fn part2(fresh_ranges: &[(usize, usize)]) -> usize {
    let new_ranges = collapse(&fresh_ranges);
    new_ranges.iter().map(|(x, y)| 1 + y - x).sum()
}
