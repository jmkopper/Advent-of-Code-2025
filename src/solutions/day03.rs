use std::error::Error;

pub fn run(input: &str) -> Result<(), Box<dyn Error>> {
    let lines: Vec<&[u8]> = input.trim().lines().map(str::as_bytes).collect();
    println!("Part 1: {}", solve(&lines, 2));
    println!("Part 2: {}", solve(&lines, 12));

    Ok(())
}

fn solve(lines: &[&[u8]], count: usize) -> usize {
    lines
        .iter()
        .map(|&line| {
            let mut cursor = line;
            let mut num = 0;

            for remaining in (0..count).rev() {
                let window_len = cursor.len().saturating_sub(remaining);
                let (idx, &digit) = cursor[..window_len]
                    .iter()
                    .enumerate()
                    .max_by(|(i1, v1), (i2, v2)| v1.cmp(v2).then(i2.cmp(i1)))
                    .expect("not enough digits");

                num = num * 10 + (digit - b'0') as usize;
                cursor = &cursor[idx + 1..];
            }
            num
        })
        .sum()
}
