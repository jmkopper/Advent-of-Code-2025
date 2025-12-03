use std::error::Error;
mod solutions;

fn main() -> Result<(), Box<dyn Error>> {
    println!("---- Day 1 ----");
    solutions::day01::run()?;
    println!("---- Day 2 ----");
    solutions::day02::run()?;
    println!("---- Day 3 ----");
    solutions::day03::run()?;
    Ok(())
}
