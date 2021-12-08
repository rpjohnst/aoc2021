use std::{error::Error, fmt, str::FromStr, ops::BitOr};

pub struct Entry {
    pub combos: [u8; 10],
    pub digits: [u8; 4],
}

impl FromStr for Entry {
    type Err = ParseEntryError;

    fn from_str(s: &str) -> Result<Entry, ParseEntryError> {
        let (combos, values) = s.split_once(" | ").ok_or(ParseEntryError)?;
        let mut groups = Iterator::chain(combos.split(' '), values.split(' '));

        let mut combos = [0; 10];
        let mut digits = [0; 4];
        for set in Iterator::chain(combos.iter_mut(), digits.iter_mut()) {
            let signals = groups.next().ok_or(ParseEntryError)?;
            *set = signals.bytes().map(|signal| 1 << (signal - b'a')).fold(0, u8::bitor);
        }

        Ok(Entry { combos, digits })
    }
}

#[derive(Debug)]
pub struct ParseEntryError;

impl Error for ParseEntryError {}

impl fmt::Display for ParseEntryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseEntryError")
    }
}
