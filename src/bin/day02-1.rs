use std::{error::Error, fs, str::FromStr};
use aoc2021::day02::Move;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day02")?;
    let moves: Vec<_> = Result::from_iter(input.split_terminator('\n').map(Move::from_str))?;

    let mut forward = 0;
    let mut down = 0;
    for command in moves {
        match command {
            Move::Forward(n) => { forward += n }
            Move::Down(n) => { down += n }
        }
    }
    println!("{}", forward * down);

    Ok(())
}
