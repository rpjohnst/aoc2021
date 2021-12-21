use std::{cmp, collections::HashMap, error::Error, fmt, fs, num::ParseIntError, str::FromStr};

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

    #[derive(Eq, PartialEq, Ord, PartialOrd, Hash)]
    struct Universe { player1: u32, score1: u32, player2: u32, score2: u32 }

    let mut universes = HashMap::new();
    universes.insert(Universe { player1, score1: 0, player2, score2: 0 }, 1usize);
    let (mut wins1, mut wins2) = (0, 0);
    while !universes.is_empty() {
        let mut new = HashMap::default();
        for (Universe { player1, score1, player2, score2 }, count) in universes {
            for a in 1..4 {
                for b in 1..4 {
                    for c in 1..4 {
                        let player1 = (player1 + a + b + c - 1) % 10 + 1;
                        let score1 = score1 + player1;
                        if score1 >= 21 {
                            wins1 += count;
                            continue;
                        }

                        for a in 1..4 {
                            for b in 1..4 {
                                for c in 1..4 {
                                    let player2 = (player2 + a + b + c - 1) % 10 + 1;
                                    let score2 = score2 + player2;
                                    if score2 >= 21 {
                                        wins2 += count;
                                        continue;
                                    }

                                    let universe = Universe { player1, score1, player2, score2 };
                                    *new.entry(universe).or_insert(0) += count;
                                }
                            }
                        }
                    }
                }
            }
        }
        universes = new;
    }
    println!("{}", cmp::max(wins1, wins2));

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
