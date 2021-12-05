use std::{collections::HashMap, error::Error, fs, iter, str::FromStr};
use aoc2021::day05::Line;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day05")?;
    let lines: Vec<_> = Result::from_iter(input.split_terminator('\n').map(Line::from_str))?;

    let mut cells = HashMap::new();
    for Line { x1, y1, x2, y2 } in lines {
        if y1 == y2 {
            for x in interval(x1, x2) { *cells.entry((x, y1)).or_insert(0) += 1; }
        } else if x1 == x2 {
            for y in interval(y1, y2) { *cells.entry((x1, y)).or_insert(0) += 1; }
        }
    }

    let danger = cells.values().filter(|&&x| x > 1).count();
    println!("{}", danger);

    Ok(())
}

fn interval(a: i32, b: i32) -> impl Iterator<Item = i32> {
    let d = i32::signum(b - a);
    iter::successors(Some(a), move |&i| if i != b { Some(i + d) } else { None })
}
