use std::{error::Error, fs, fmt, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day17")?;
    let input = input.strip_prefix("target area: x=").ok_or(ParseTargetError)?;
    let (x1, input) = input.split_once("..").ok_or(ParseTargetError)?;
    let _x1 = i32::from_str(x1)?;
    let (x2, input) = input.split_once(", y=").ok_or(ParseTargetError)?;
    let _x2 = i32::from_str(x2)?;
    let (y1, input) = input.split_once("..").ok_or(ParseTargetError)?;
    let y1 = i32::from_str(y1)?;
    let (y2, input) = input.split_once("\n").ok_or(ParseTargetError)?;
    let _y2 = i32::from_str(y2)?;
    match input { "" => {}, _ => Err(ParseTargetError)? };

    let vy = -y1 - 1;
    println!("{}", vy * (vy + 1) / 2);

    Ok(())
}

#[derive(Debug)]
pub struct ParseTargetError;

impl Error for ParseTargetError {}

impl fmt::Display for ParseTargetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseBitsError")
    }
}
