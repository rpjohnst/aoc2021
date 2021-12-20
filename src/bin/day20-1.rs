use std::{error::Error, fs};
use aoc2021::day20::{Image, ParseImageError, pixel_from_byte};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day20")?;
    let mut lines = input.split_terminator('\n');
    let algorithm = <Result<Vec<_>, _>>::from_iter(
        lines.next().ok_or(ParseImageError)?.bytes().map(pixel_from_byte))?;
    match lines.next() { Some(empty) if empty.is_empty() => {}, _ => { Err(ParseImageError)? } };
    let mut len = 0;
    let pixels = <Result<Vec<_>, _>>::from_iter(lines.flat_map(|line| {
        len = line.len() as isize;
        line.bytes().map(pixel_from_byte)
    }))?;
    let outside = false;

    let image = Image { pixels, len, outside };
    let image = image.enhance(&algorithm[..]);
    let image = image.enhance(&algorithm[..]);
    println!("{}", image.pixels.into_iter().filter(|&p| p).count());

    Ok(())
}
