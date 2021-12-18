use std::{error::Error, fs, str::FromStr};
use aoc2021::day18::{Number, ParseNumberError};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day18")?;
    let numbers: Vec<_> = Result::from_iter(input.split_terminator('\n').map(Number::from_str))?;

    let mut numbers = numbers.into_iter();
    let mut sum = numbers.next().ok_or(ParseNumberError)?;
    for number in numbers { sum += number; }

    let magnitude = sum.magnitude();
    println!("{}", magnitude);

    Ok(())
}
