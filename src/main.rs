use std::error::Error;
mod solutions;

fn main() -> Result<(), Box<dyn Error>> {
    let puzzles = vec![
        solutions::day01::run,
        solutions::day02::run,
        solutions::day03::run,
        solutions::day04::run,
        solutions::day05::run,
        solutions::day06::run,
    ];

    for (i, puzzle) in puzzles.iter().enumerate() {
        let input_file = format!("puzzles/day{}.txt", i + 1);
        let input = std::fs::read_to_string(input_file)?;
        println!("---- Day {} ----", i + 1);
        puzzle(&input)?;
    }
    Ok(())
}
