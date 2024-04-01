use std::{error::Error, io::stdin};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::with_capacity(1000);
    stdin().read_line(&mut input)?;

    let mut leaderboard = [0u64; 4];

    let mut input_chars = input.trim().chars();
    let mut previous = char_to_index(input_chars.next().unwrap());
    let mut repetitions: u64 = 1;

    leaderboard[previous] = repetitions;

    for input_char in input_chars {
        let current = char_to_index(input_char);
        let score = &mut leaderboard[previous];

        if previous == current {
            repetitions += 1;
        } else {
            repetitions = 1;
        }

        previous = current;
        *score = repetitions.max(*score);
    }
    println!("{}", leaderboard.iter().max().unwrap());

    Ok(())
}

fn char_to_index(value: char) -> usize {
    match value {
        'A' => 0,
        'C' => 1,
        'G' => 2,
        'T' => 3,
        _ => unreachable!(),
    }
}
