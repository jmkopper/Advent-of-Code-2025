use std::error::Error;

pub fn run(input: &str) -> Result<(), Box<dyn Error>> {
    let (first, second) = input.split_once("\n\n").ok_or("where chunks")?;

    let mut fresh_ranges = parse_ranges(first)?;
    fresh_ranges.sort_unstable(); // tuples are sorted by dictionary order
    let collapsed = collapse(&fresh_ranges); // collapsing ranges also speeds up part 1
    let ingredient_ids: Vec<usize> = second.lines().map(str::parse).collect::<Result<_, _>>()?;

    println!("Part 1: {}", part1(&collapsed, &ingredient_ids));
    println!("Part 2: {}", part2(&collapsed));

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

fn part2(fresh_ranges: &[(usize, usize)]) -> usize {
    fresh_ranges.iter().map(|(x, y)| 1 + y - x).sum()
}
