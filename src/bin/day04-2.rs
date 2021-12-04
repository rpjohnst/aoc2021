#![feature(array_chunks)]

use std::{error::Error, fmt, fs, iter, mem, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day04")?;
    let (numbers, boards) = input.split_once('\n').ok_or(ParseBingoError)?;
    let numbers: Vec<_> = Result::from_iter(numbers.split(',').map(u32::from_str))?;
    let cells: Vec<_> = Result::from_iter(boards.split_ascii_whitespace().map(u32::from_str))?;
    let rows: Vec<[u32; 5]> = Vec::from_iter(cells.array_chunks().copied());
    let boards: Vec<[[u32; 5]; 5]> = Vec::from_iter(rows.array_chunks().copied());

    let mut marks = Vec::from_iter(iter::repeat([[false; 5]; 5]).take(boards.len()));
    let mut won = Vec::from_iter(iter::repeat(false).take(boards.len()));
    let mut live = boards.len();
    let mut last = 0;
    let mut winner = None;
    'game: for number in numbers {
        for (b, board) in boards.iter().enumerate() {
            for (i, row) in board.iter().enumerate() {
                for (j, &cell) in row.iter().enumerate() {
                    if number == cell {
                        marks[b][i][j] = true;
                        if
                            marks[b][i].iter().all(|&c| c) ||
                            marks[b].iter().map(|r| &r[j]).all(|&c| c)
                        {
                            if !mem::replace(&mut won[b], true) {
                                live -= 1;
                                if live == 0 {
                                    last = number;
                                    winner = Some(b);
                                    break 'game;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if let Some(winner) = winner {
        let mut score = 0;
        for (i, row) in boards[winner].iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if !marks[winner][i][j] { score += cell; }
            }
        }
        println!("{}", score * last);
    }

    Ok(())
}

#[derive(Debug)]
pub struct ParseBingoError;

impl Error for ParseBingoError {}

impl fmt::Display for ParseBingoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseBingoError")
    }
}
