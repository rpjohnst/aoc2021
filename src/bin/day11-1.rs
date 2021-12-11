use std::{cmp, collections::HashSet, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day11")?;
    let mut len = 0;
    let mut levels = Vec::from_iter(input.split_terminator('\n').flat_map(|line| {
        len = line.len();
        line.bytes().map(|x| x - b'0')
    }));
    let mut grid = Vec::from_iter(levels.chunks_mut(len));

    let mut count = 0;
    let mut flash = HashSet::new();
    let mut stack = Vec::default();
    for _ in 0..100 {
        for y in 0..len {
            for x in 0..len {
                grid[y][x] += 1;
                if grid[y][x] > 9 && flash.insert((x, y)) { stack.push((x, y)); }
            }
        }
        while let Some((x, y)) = stack.pop() {
            for y in y.saturating_sub(1)..=cmp::min(y + 1, len - 1) {
                for x in x.saturating_sub(1)..=cmp::min(x + 1, len - 1) {
                    grid[y][x] += 1;
                    if grid[y][x] > 9 && flash.insert((x, y)) { stack.push((x, y)); }
                }
            }
        }
        count += flash.len();
        for (x, y) in flash.drain() {
            grid[y][x] = 0;
        }
    }
    println!("{:?}", count);

    Ok(())
}
