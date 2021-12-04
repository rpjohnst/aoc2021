#![feature(array_windows)]

use std::{error::Error, fs, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day01")?;
    let depths: Vec<_> = Result::from_iter(input.split_terminator('\n').map(u32::from_str))?;

    let count = depths.array_windows().filter(|&[a, b]| a < b).count();
    println!("{}", count);

    Ok(())
}
