use std::error::Error;

pub fn run(input: &str) -> Result<(), Box<dyn Error>> {
    let ranges: Vec<(u64, u64)> = input
        .trim()
        .split(',')
        .map(|s| {
            let (start, end) = s.split_once("-").ok_or("Invalid range format")?;
            Ok((start.parse()?, end.parse()?))
        })
        .collect::<Result<_, Box<dyn Error>>>()?;

    println!("Part 1: {}", part1(&ranges));
    println!("Part 2: {}", part2(&ranges));

    Ok(())
}

fn part1(ranges: &[(u64, u64)]) -> u64 {
    ranges
        .iter()
        .flat_map(|&(start, end)| start..=end)
        .filter(|&x| is_double(x))
        .sum()
}

fn part2(ranges: &[(u64, u64)]) -> u64 {
    ranges
        .iter()
        .flat_map(|&(start, end)| start..=end)
        .filter(|&x| is_pattern(x))
        .sum()
}

fn is_double(x: u64) -> bool {
    let digits = x.checked_ilog10().unwrap_or(0) + 1;
    if digits % 2 != 0 {
        return false;
    }
    let divisor = 10u64.pow(digits / 2);
    x / divisor == x % divisor
}

fn is_pattern(x: u64) -> bool {
    let mut buf = [0u8; 20];
    let mut temp = x;
    let mut len = 0;

    while temp > 0 {
        len += 1;
        buf[20 - len] = (temp % 10) as u8;
        temp /= 10;
    }

    let digits = &buf[20 - len..];
    (1..=len / 2).any(|pat_len| {
        if len % pat_len != 0 {
            return false;
        }
        let pattern = &digits[..pat_len];
        digits.chunks_exact(pat_len).all(|chunk| chunk == pattern)
    })
}
