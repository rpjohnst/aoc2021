use std::{error::Error, fmt, fs, num::ParseIntError, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day21")?;
    let mut lines = input.split_terminator('\n');
    let line = lines.next().ok_or(ParseGameError)?;
    let (_, player1) = line.split_once("Player 1 starting position: ").ok_or(ParseGameError)?;
    let player1 = u32::from_str(player1)?;
    let line = lines.next().ok_or(ParseGameError)?;
    let (_, player2) = line.split_once("Player 2 starting position: ").ok_or(ParseGameError)?;
    let player2 = u32::from_str(player2)?;
    match lines.next() { None => {}, Some(_) => Err(ParseGameError)? }

    let (mut player1, mut player2) = (player1, player2);
    let (mut score1, mut score2) = (0, 0);
    let mut die = 1;
    let mut rolls = 0;
    let loser = loop {
        let a = die;
        die = die % 100 + 1;
        let b = die;
        die = die % 100 + 1;
        let c = die;
        die = die % 100 + 1;
        rolls += 3;

        player1 = (player1 + a + b + c - 1) % 10 + 1;
        score1 += player1;
        if score1 >= 1000 { break score2; }

        let a = die;
        die = die % 100 + 1;
        let b = die;
        die = die % 100 + 1;
        let c = die;
        die = die % 100 + 1;
        rolls += 3;

        player2 = (player2 + a + b + c - 1) % 10 + 1;
        score2 += player2;
        if score2 >= 1000 { break score1; }
    };
    println!("{}", loser * rolls);

    Ok(())
}

#[derive(Debug)]
pub struct ParseGameError;

impl From<ParseIntError> for ParseGameError {
    fn from(_: ParseIntError) -> Self { ParseGameError }
}

impl Error for ParseGameError {}

impl fmt::Display for ParseGameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseGameError")
    }
}
