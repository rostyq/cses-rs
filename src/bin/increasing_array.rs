use std::{error::Error, io::stdin};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::with_capacity(1000);
    stdin().read_line(&mut input)?;

    let n: usize = input.trim().parse()?;
    debug_assert!(n >= 1 && n <= 200_000);

    input.clear();
    stdin().read_line(&mut input)?;

    let mut moves = 0u64;
    let mut x_ = 0u64;
    for x in input.split_whitespace().take(n) {
        let x = x.parse::<u64>()?;
        debug_assert!(x >= 1 && x <= 1_000_000_000);

        if let Some(step) = x_.checked_sub(x) {
            moves += step;
            x_ = x + step;
        } else {
            x_ = x;
        }
    }

    println!("{:?}", moves);

    Ok(())
}
