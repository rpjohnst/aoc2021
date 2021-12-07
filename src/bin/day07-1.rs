use std::{error::Error, fs, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day07")?;
    let crabs: Vec<_> = Result::from_iter(input.split_terminator(&[',', '\n']).map(i32::from_str))?;

    let min = crabs.iter().copied().min().unwrap_or(0);
    let max = crabs.iter().copied().max().unwrap_or(0);
    let cost = (min..=max).map(|x| {
        crabs.iter().map(|&crab| i32::abs(x - crab)).sum()
    }).min().unwrap_or(0);
    println!("{}", cost);

    Ok(())
}
