use std::{error::Error, fs, str::FromStr};
use aoc2021::day08::Entry;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day08")?;
    let entries: Vec<_> = Result::from_iter(input.split_terminator('\n').map(Entry::from_str))?;

    let mut count = 0;
    for Entry { digits, .. } in entries {
        count += digits.into_iter().filter(|&set| [2, 4, 3, 7].contains(&set.count_ones())).count();
    }
    println!("{}", count);

    Ok(())
}
