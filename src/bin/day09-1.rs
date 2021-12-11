#![feature(array_windows, iter_zip)]

use std::{cmp, error::Error, fs, iter};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day09")?;
    let mut len = 0;
    let heights = Vec::from_iter(input.split_terminator('\n').flat_map(|line| {
        len = line.len();
        line.bytes().map(|x| x - b'0')
    }));

    let mut grid = Vec::with_capacity((len + 2) * (len + 2));
    grid.extend(iter::repeat(u8::MAX).take(len + 2));
    for row in heights.chunks(len) {
        grid.push(u8::MAX);
        grid.extend(row.iter().copied());
        grid.push(u8::MAX);
    }
    grid.extend(iter::repeat(u8::MAX).take(len + 2));
    let grid = Vec::from_iter(grid.chunks(len + 2));

    let mut risk = 0;
    for y in (0..len).map(|y| y + 1) {
        for x in (0..len).map(|x| x + 1) {
            let height = grid[y][x];
            let left = grid[y][x - 1];
            let right = grid[y][x + 1];
            let up = grid[y - 1][x];
            let down = grid[y + 1][x];
            if height < cmp::min(cmp::min(left, right), cmp::min(up, down)) {
                risk += 1 + height as usize;
            }
        }
    }
    println!("{}", risk);

    Ok(())
}
