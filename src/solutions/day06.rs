use std::error::Error;

pub fn run(input: &str) -> Result<(), Box<dyn Error>> {
    let mut columns: Vec<Vec<usize>> = Vec::new();
    let mut operations: Vec<u8> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_ascii_whitespace().peekable();
        if parts.peek().is_none() {
            continue;
        }
        let first_token = parts.peek().unwrap();
        if matches!(first_token.chars().next(), Some('*') | Some('+')) {
            operations = parts.map(|c| c.as_bytes()[0]).collect();
            continue;
        }

        let nums: Vec<usize> = parts
            .map(|p| p.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;

        for (i, &n) in nums.iter().enumerate() {
            if i >= columns.len() {
                columns.push(Vec::new());
            }
            columns[i].push(n);
        }
    }

    println!("Part 1: {}", part1(&columns, &operations));
    println!("Part 2: {}", part2(input));

    Ok(())
}

fn part1(columns: &[Vec<usize>], operations: &[u8]) -> usize {
    let mut total = 0;
    for (i, column) in columns.iter().enumerate() {
        match operations.get(i).copied() {
            Some(b'+') => total += column.iter().sum::<usize>(),
            Some(b'*') => total += column.iter().product::<usize>(),
            _ => {}
        }
    }
    total
}

fn parse_for_part2(input: &str) -> Vec<Vec<u8>> {
    let mut parsed: Vec<Vec<u8>> = Vec::new();
    let mut longest = 0;

    for line in input.lines() {
        let bytes = line.as_bytes().to_vec();
        longest = longest.max(bytes.len());
        parsed.push(bytes);
    }

    // pad and reverse rows
    for row in &mut parsed {
        row.resize(longest, b' ');
        row.reverse();
    }
    parsed
}

fn part2(input: &str) -> usize {
    let mut total = 0;
    let parsed = parse_for_part2(input);
    let ops = &parsed[parsed.len() - 1];

    let mut to_process: Vec<usize> = Vec::new();
    for col in 0..parsed[0].len() {
        let mut cur = 0;

        for row in 0..parsed.len() - 1 {
            let b = parsed[row][col];
            if b.is_ascii_digit() {
                cur = cur * 10 + (b - b'0') as usize;
            }
        }

        if cur > 0 {
            to_process.push(cur);
        }

        // it appears that the operation is always left-aligned
        // so we can add/multiply as soon as we see an operation
        if ops[col] == b'+' {
            total += to_process.iter().sum::<usize>();
            to_process.clear();
        } else if ops[col] == b'*' {
            total += to_process.iter().product::<usize>();
            to_process.clear();
        }
    }
    total
}
