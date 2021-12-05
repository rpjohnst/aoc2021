use std::{error::Error, fmt, num::ParseIntError, str::FromStr};

pub struct Line {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl FromStr for Line {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Line, ParseLineError> {
        let (a, b) = s.split_once(" -> ").ok_or(ParseLineError)?;
        let (x1, y1) = a.split_once(',').ok_or(ParseLineError)?;
        let (x2, y2) = b.split_once(',').ok_or(ParseLineError)?;
        Ok(Line {
            x1: i32::from_str(x1)?, y1: i32::from_str(y1)?,
            x2: i32::from_str(x2)?, y2: i32::from_str(y2)?,
        })
    }
}

#[derive(Debug)]
pub struct ParseLineError;

impl From<ParseIntError> for ParseLineError {
    fn from(_: ParseIntError) -> Self { ParseLineError }
}

impl Error for ParseLineError {}

impl fmt::Display for ParseLineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseLineError")
    }
}
