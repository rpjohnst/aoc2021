use std::{collections::HashSet, error::Error, fmt, fs, iter, num::ParseIntError, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day13")?;
    let mut lines = input.split_terminator('\n');
    let dots = lines.by_ref().take_while(|&line| !line.is_empty());
    let dots = <Result<Vec<_>, ParsePageError>>::from_iter(dots.map(|dot| {
        let (x, y) = dot.split_once(',').ok_or(ParsePageError)?;
        Ok((i32::from_str(x)?, i32::from_str(y)?))
    }))?;
    let folds = <Result<Vec<_>, ParsePageError>>::from_iter(lines.map(|fold| {
        let fold = fold.strip_prefix("fold along ").ok_or(ParsePageError)?;
        let (axis, v) = fold.split_once('=').ok_or(ParsePageError)?;
        let axis = match axis { "x" => Fold::X, "y" => Fold::Y, _ => Err(ParsePageError)? };
        Ok(axis(i32::from_str(v)?))
    }))?;

    let mut dots = <HashSet<_>>::from_iter(dots);
    for fold in folds {
        dots = HashSet::from_iter(dots.into_iter().map(|(x, y)| {
            match fold {
                Fold::X(fold) => if fold < x { (fold - x + fold, y) } else { (x, y) }
                Fold::Y(fold) => if fold < y { (x, fold - y + fold) } else { (x, y) }
            }
        }));
    }

    let width = dots.iter().map(|&(x, _)| x + 1).max().unwrap_or(0) as usize;
    let height = dots.iter().map(|&(_, y)| y + 1).max().unwrap_or(0) as usize;
    let mut code = Vec::from_iter(iter::repeat(false).take(width * height));
    for &(x, y) in &dots { code[y as usize * width + x as usize] = true; }
    for row in code.chunks(width) {
        for &dot in row { print!("{}", if dot { '#' } else { '.' }); }
        println!();
    }

    Ok(())
}

enum Fold { X(i32), Y(i32) }

#[derive(Debug)]
pub struct ParsePageError;

impl From<ParseIntError> for ParsePageError {
    fn from(_: ParseIntError) -> Self { ParsePageError }
}

impl Error for ParsePageError {}

impl fmt::Display for ParsePageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParsePageError")
    }
}
