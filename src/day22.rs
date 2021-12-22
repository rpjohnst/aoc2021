use std::{error::Error, fmt, num::ParseIntError, str::FromStr};

pub struct Step { pub on: bool, pub x: Axis, pub y: Axis, pub z: Axis }

#[derive(Copy, Clone)]
pub struct Axis { pub start: i64, pub end: i64 }

impl FromStr for Step {
    type Err = ParseStepError;

    fn from_str(s: &str) -> Result<Step, ParseStepError> {
        let (on, s) = s.split_once(" x=").ok_or(ParseStepError)?;
        let on = match on { "on" => true, "off" => false, _ => Err(ParseStepError)? };
        let (x1, s) = s.split_once("..").ok_or(ParseStepError)?;
        let (x2, s) = s.split_once(",y=").ok_or(ParseStepError)?;
        let x = Axis { start: i64::from_str(x1)?, end: i64::from_str(x2)? };
        let (y1, s) = s.split_once("..").ok_or(ParseStepError)?;
        let (y2, s) = s.split_once(",z=").ok_or(ParseStepError)?;
        let y = Axis { start: i64::from_str(y1)?, end: i64::from_str(y2)? };
        let (z1, z2) = s.split_once("..").ok_or(ParseStepError)?;
        let z = Axis { start: i64::from_str(z1)?, end: i64::from_str(z2)? };
        Ok(Step { on, x, y, z })
    }
}

#[derive(Debug)]
pub struct ParseStepError;

impl From<ParseIntError> for ParseStepError {
    fn from(_: ParseIntError) -> Self { ParseStepError }
}

impl Error for ParseStepError {}

impl fmt::Display for ParseStepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseStepError")
    }
}
