use std::{error::Error, io::stdin};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::with_capacity(1000);
    stdin().read_line(&mut input)?;

    let n: u64 = input.trim().parse()?;
    debug_assert!(n >= 2 && n <= 200_000);

    let n_sum: u64 = (1..=n).sum();

    input.clear();
    stdin().read_line(&mut input)?;

    let mut numbers_sum = 0;
    for value in input.trim().split_whitespace() {
        let value = value.parse::<u64>()?;
        debug_assert!(value <= n);

        numbers_sum += value;
    }

    debug_assert!(numbers_sum < n_sum);
    println!("{}", n_sum - numbers_sum);

    Ok(())
}
