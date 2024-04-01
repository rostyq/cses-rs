use std::{io::{stdin, stdout, Write}, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::with_capacity(100);
    stdin().read_line(&mut input)?;

    let mut n: u64 = input.trim().parse()?;
    debug_assert!(n > 0 && n < 1_000_000);

    let mut out = stdout();

    loop {
        write!(&mut out, "{}", n)?;

        if n == 1 {
            break;
        } else {
            write!(&mut out, " ")?;
        }

        if n % 2 == 0 {
            n /= 2
        } else {
            n *= 3;
            n += 1;
        }
    }

    out.write("\n".as_bytes())?;
    out.flush()?;

    Ok(())
}
