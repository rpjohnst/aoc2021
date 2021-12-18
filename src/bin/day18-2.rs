#![feature(array_windows, iter_zip)]

use std::{error::Error, fs, str::FromStr};
use aoc2021::day18::Number;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day18")?;
    let numbers: Vec<_> = Result::from_iter(input.split_terminator('\n').map(Number::from_str))?;

    let mut max = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            let mut sum = numbers[i].clone();
            sum += numbers[j].clone();

            let magnitude = sum.magnitude();
            if magnitude > max { max = magnitude; }
        }
    }
    println!("{}", max);

    Ok(())
}
