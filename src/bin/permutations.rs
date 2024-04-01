use std::{error::Error, io::stdin};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::with_capacity(1000);
    stdin().read_line(&mut input)?;

    let n: usize = input.trim().parse()?;
    debug_assert!(n >= 1 && n <= 1_000_000);

    // TODO:
    println!("NO SOLUTION");

    Ok(())
}