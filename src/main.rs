use std::error::Error;
mod solutions;

fn main() -> Result<(), Box<dyn Error>> {
    solutions::day01::run()?;
    solutions::day02::run()?;
    Ok(())
}
