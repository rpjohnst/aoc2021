use std::{error::Error, fmt, num::ParseIntError, str::FromStr};

pub enum Move {
    Forward(i32),
    Down(i32),
}

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Move, ParseMoveError> {
        if let Some(n) = s.strip_prefix("forward ") {
            Ok(Move::Forward(i32::from_str(n)?))
        } else if let Some(n) = s.strip_prefix("down ") {
            Ok(Move::Down(i32::from_str(n)?))
        } else if let Some(n) = s.strip_prefix("up ") {
            Ok(Move::Down(-i32::from_str(n)?))
        } else {
            Err(ParseMoveError)
        }
    }
}

#[derive(Debug)]
pub struct ParseMoveError;

impl From<ParseIntError> for ParseMoveError {
    fn from(_: ParseIntError) -> Self { ParseMoveError }
}

impl Error for ParseMoveError {}

impl fmt::Display for ParseMoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseMoveError")
    }
}
