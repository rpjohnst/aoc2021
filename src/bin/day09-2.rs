#![feature(array_windows, iter_zip)]

use std::{collections::HashSet, cmp, error::Error, fs, iter};

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

    let mut basins = Vec::default();
    for y in (0..len).map(|y| y + 1) {
        for x in (0..len).map(|x| x + 1) {
            let height = grid[y][x];
            let left = grid[y][x - 1];
            let right = grid[y][x + 1];
            let up = grid[y - 1][x];
            let down = grid[y + 1][x];
            if height < cmp::min(cmp::min(left, right), cmp::min(up, down)) {
                basins.push(basin(&grid[..], x, y));
            }
        }
    }

    basins.sort_unstable();
    if let &[.., a, b, c] = &basins[..] {
        println!("{}", a * b * c);
    }

    Ok(())
}

fn basin(grid: &[&[u8]], x: usize, y: usize) -> usize {
    let mut stack = Vec::default();

    let mut visit = HashSet::new();
    if visit.insert((x, y)) { stack.push((x, y)); }

    while let Some((x, y)) = stack.pop() {
        let height = grid[y][x];

        let left = grid[y][x - 1];
        if height < left && left < 9 && visit.insert((x - 1, y)) { stack.push((x - 1, y)); }

        let right = grid[y][x + 1];
        if height < right && right < 9 && visit.insert((x + 1, y)) { stack.push((x + 1, y)); }

        let up = grid[y - 1][x];
        if height < up && up < 9 && visit.insert((x, y - 1)) { stack.push((x, y - 1)); }

        let down = grid[y + 1][x];
        if height < down && down < 9 && visit.insert((x, y + 1)) { stack.push((x, y + 1)); }
    }

    visit.len()
}
